
// types
const xltypeNum: i32 = 1;
const xltypeStr: i32 = 2;
const xltypeBool: i32 = 4;
const xltypeRef: i32 = 8;
const xltypeErr: i32 = 16;
const xltypeFlow: i32 = 32;
const xltypeMulti: i32 = 64;
const xltypeMissing: i32 = 128;
const xltypeNil: i32 = 256;
const xltypeSRef: i32 = 1024;
const xltypeInt: i32 = 2048;

// free bit flag, shouldn't be needed
const xlbitXLFree: i32 = 4096;
const xlbitDLLFree: i32 = 16384;
const xltypeBigData: i32 = 2050;

// error codes for XLOPER12
const xlerrNull: i32 = 0;
const xlerrDiv0: i32 = 7;
const xlerrValue: i32 = 15;
const xlerrRef: i32 = 23;
const xlerrName: i32 = 29;
const xlerrNum: i32 = 36;
const xlerrNA: i32 = 42;
const xlerrGettingData: i32 = 43;

// typedefs from C, many unneeded

pub type wchar_t = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type INT32 = ::std::os::raw::c_int;
pub type ULONG_PTR = ::std::os::raw::c_ulonglong;
pub type DWORD_PTR = ULONG_PTR;
pub type CHAR = ::std::os::raw::c_char;
pub type LONG = ::std::os::raw::c_long;
pub type WCHAR = wchar_t;
pub type LPSTR = *mut CHAR;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type POINT = tagPOINT;
pub type XCHAR = WCHAR;
pub type RW = INT32;
pub type COL = INT32;
pub type IDSHEET = DWORD_PTR;

// not including FP12 for now, think it's better to run everything through XLOPER12 for v1
// XLOPER12
// TODO: Clean up mangled names from bindgen for clarity
// TODO: Fix public / private access modifiers, bindgen pub by default

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xloper12 {
    pub val: xloper12__bindgen_ty_1,
    pub xltype: DWORD,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union xloper12__bindgen_ty_1 {
    pub num: f64,
    pub str_: *mut XCHAR,
    pub xbool: BOOL,
    pub err: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub sref: xloper12__bindgen_ty_1__bindgen_ty_1,
    pub mref: xloper12__bindgen_ty_1__bindgen_ty_2,
    pub array: xloper12__bindgen_ty_1__bindgen_ty_3,
    pub flow: xloper12__bindgen_ty_1__bindgen_ty_4,
    pub bigdata: xloper12__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct xloper12__bindgen_ty_1__bindgen_ty_1 {
    pub count: WORD,
    pub ref_: XLREF12,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_1>() - 20usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_1::count"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_1, count) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_1::ref_"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_1, ref_) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xloper12__bindgen_ty_1__bindgen_ty_2 {
    pub lpmref: *mut XLMREF12,
    pub idSheet: IDSHEET,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_2"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_2>() - 16usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_2"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_2>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_2::lpmref"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_2, lpmref) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_2::idSheet"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_2, idSheet) - 8usize];
};
impl Default for xloper12__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xloper12__bindgen_ty_1__bindgen_ty_3 {
    pub lparray: *mut xloper12,
    pub rows: RW,
    pub columns: COL,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_3"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_3>() - 16usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_3"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_3>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_3::lparray"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_3, lparray) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_3::rows"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_3, rows) - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_3::columns"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_3, columns) - 12usize];
};
impl Default for xloper12__bindgen_ty_1__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xloper12__bindgen_ty_1__bindgen_ty_4 {
    pub valflow: xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1,
    pub rw: RW,
    pub col: COL,
    pub xlflow: BYTE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1 {
    pub level: ::std::os::raw::c_int,
    pub tbctrl: ::std::os::raw::c_int,
    pub idSheet: IDSHEET,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1>() - 8usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1::level"][::std::mem::offset_of!(
        xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1,
        level
    ) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1::tbctrl"][::std::mem::offset_of!(
        xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1,
        tbctrl
    ) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1::idSheet"][::std::mem::offset_of!(
        xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1,
        idSheet
    ) - 0usize];
};
impl Default for xloper12__bindgen_ty_1__bindgen_ty_4__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_4"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_4>() - 24usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_4"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_4>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4::valflow"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_4, valflow) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4::rw"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_4, rw) - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4::col"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_4, col) - 12usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_4::xlflow"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_4, xlflow) - 16usize];
};
impl Default for xloper12__bindgen_ty_1__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xloper12__bindgen_ty_1__bindgen_ty_5 {
    pub h: xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
    pub cbData: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    pub lpbData: *mut BYTE,
    pub hdata: HANDLE,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1>() - 8usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1::lpbData"][::std::mem::offset_of!(
        xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
        lpbData
    ) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1::hdata"][::std::mem::offset_of!(
        xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
        hdata
    ) - 0usize];
};
impl Default for xloper12__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1__bindgen_ty_5"]
        [::std::mem::size_of::<xloper12__bindgen_ty_1__bindgen_ty_5>() - 16usize];
    ["Alignment of xloper12__bindgen_ty_1__bindgen_ty_5"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1__bindgen_ty_5>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_5::h"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_5, h) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1__bindgen_ty_5::cbData"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1__bindgen_ty_5, cbData) - 8usize];
};
impl Default for xloper12__bindgen_ty_1__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12__bindgen_ty_1"][::std::mem::size_of::<xloper12__bindgen_ty_1>() - 24usize];
    ["Alignment of xloper12__bindgen_ty_1"]
        [::std::mem::align_of::<xloper12__bindgen_ty_1>() - 8usize];
    ["Offset of field: xloper12__bindgen_ty_1::num"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, num) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::str_"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, str_) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::xbool"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, xbool) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::err"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, err) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::w"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, w) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::sref"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, sref) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::mref"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, mref) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::array"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, array) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::flow"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, flow) - 0usize];
    ["Offset of field: xloper12__bindgen_ty_1::bigdata"]
        [::std::mem::offset_of!(xloper12__bindgen_ty_1, bigdata) - 0usize];
};
impl Default for xloper12__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xloper12"][::std::mem::size_of::<xloper12>() - 32usize];
    ["Alignment of xloper12"][::std::mem::align_of::<xloper12>() - 8usize];
    ["Offset of field: xloper12::val"][::std::mem::offset_of!(xloper12, val) - 0usize];
    ["Offset of field: xloper12::xltype"][::std::mem::offset_of!(xloper12, xltype) - 24usize];
};
impl Default for xloper12 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type XLOPER12 = xloper12;
pub type LPXLOPER12 = *mut xloper12;

// Excel12v callback, only one we'll need since Rust doesn't support varargs without macros

unsafe extern "C" {
    pub fn Excel12v(
        xlfn: ::std::os::raw::c_int,
        operRes: LPXLOPER12,
        count: ::std::os::raw::c_int,
        opers: *mut LPXLOPER12,
    ) -> ::std::os::raw::c_int;
}