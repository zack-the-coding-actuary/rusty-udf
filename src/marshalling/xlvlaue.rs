

// Error codes to/from Excel
pub enum ErrCode {
    Null,
    Div0,
    Value,
    Ref,
    Name,
    Num,
    NA,
    GettingData,
}

// All actual coerced values possible from Excel
pub enum XLType {
    Num(double),
    Str(String),
    Bool(bool),
    ErrCode(ErrCode),
    Missing,
}

// Coerced values can be either a range of values, or a plain value type
pub enum XLValue {
    Value(XLType),
    Range { rows: i32, cols: i32, values: Box<[XLType]>},
}