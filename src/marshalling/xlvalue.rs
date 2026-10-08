use crate::marshalling::xlcall::{
    XLERR_DIV0, XLERR_GETTING_DATA, XLERR_NA, XLERR_NAME,
    XLERR_NULL, XLERR_NUM, XLERR_REF, XLERR_VALUE, XLOPER12,
    Xloper12Array, XLTYPE_BOOL, XLTYPE_ERR, XLTYPE_MISSING,
    XLTYPE_MULTI, XLTYPE_NUM, XLTYPE_STR,
};
use std::convert::From;

// Error codes to/from Excel
pub enum ErrCode {
    Null,
    Div0,
    Value,
    Ref,
    Name,
    Num,
    Na,
    GettingData,
}

// All actual coerced values possible from Excel
// Maybe make implementation private? Must access through XLValue
pub enum XLType {
    Num(f64),
    Str(String),
    Bool(bool),
    ErrCode(ErrCode),
    Missing,
}

// Coerced values can be either a range of values, or a plain value type
pub enum XLValue {
    Value(XLType),
    Range(XLRange),
}

// Rectangular block of cells, stored row-major like Excel's own array layout
pub struct XLRange {
    rows: usize,
    cols: usize,
    values: Box<[XLType]>,
}

impl XLRange {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&XLType> {
        (row < self.rows && col < self.cols).then(|| &self.values[row * self.cols + col])
    }

    // Row slices come for free from the row-major layout
    pub fn rows_iter(&self) -> impl Iterator<Item = &[XLType]> {
        // chunks panics on a chunk size of 0, which an empty range would hit
        self.values.chunks(self.cols.max(1))
    }
}

impl std::ops::Index<(usize, usize)> for XLRange {
    type Output = XLType;

    fn index(&self, (row, col): (usize, usize)) -> &XLType {
        assert!(row < self.rows && col < self.cols, "range index out of bounds");
        &self.values[row * self.cols + col]
    }
}

impl From<Xloper12Array> for XLRange {
    fn from(array: Xloper12Array) -> Self {
        if array.lparray.is_null() || array.rows < 0 || array.columns < 0 {
            panic!("Excel array came through malformed");
        }

        let (rows, cols) = (array.rows as usize, array.columns as usize);
        let cells = unsafe { std::slice::from_raw_parts(array.lparray, rows * cols) };
        let values = cells
            .iter()
            .map(|cell| match XLValue::from(*cell) {
                XLValue::Value(v) => v,
                XLValue::Range(_) => panic!("Excel arrays cannot be nested"),
            })
            .collect();

        XLRange { rows, cols, values }
    }
}

// Assign error codes from XLOPER12
impl From<i32> for ErrCode {
    fn from(err: i32) -> Self {
        match err {
            XLERR_NULL => ErrCode::Null,
            XLERR_DIV0 => ErrCode::Div0,
            XLERR_VALUE => ErrCode::Value,
            XLERR_REF => ErrCode::Ref,
            XLERR_NAME => ErrCode::Name,
            XLERR_NUM => ErrCode::Num,
            XLERR_NA => ErrCode::Na,
            XLERR_GETTING_DATA => ErrCode::GettingData,
            // panic here should never happen
            _ => panic!("Invalid error code from Excel"),
        }
    }
}

// Implement Rust idiomatic enum marshalling method from C tagged union
impl From<XLOPER12> for XLValue {
    fn from(value: XLOPER12) -> Self {
        match value.xltype {
            XLTYPE_NUM => XLValue::Value(XLType::Num(unsafe { value.val.num })),

            // Truncates exotic characters that don't fit in u8
            XLTYPE_STR => {
                let p = unsafe { value.val.str_ };

                // pointer should NEVER be null, okay panicking here
                if p.is_null() {
                    panic!("Excel string came through as null pointer");
                }
                let len = unsafe { *p } as usize;
                let units: &[u16] = unsafe { std::slice::from_raw_parts(p.add(1), len) };
                XLValue::Value(XLType::Str(String::from_utf16_lossy(units)))
            },
            XLTYPE_BOOL => {
                let b = unsafe { value.val.xbool } != 0;
                XLValue::Value(XLType::Bool(b))
            }
            XLTYPE_ERR => XLValue::Value(XLType::ErrCode(ErrCode::from( unsafe { value.val.err } ))),
            XLTYPE_MISSING => XLValue::Value(XLType::Missing),
            XLTYPE_MULTI => XLValue::Range(XLRange::from(unsafe { value.val.array })),
            _ => panic!("Can only marshal coerced value types from Excel to rusty-udf"),
        }
    }
}