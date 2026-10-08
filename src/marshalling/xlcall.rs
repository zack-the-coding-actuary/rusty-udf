
// types
pub const XLTYPE_NUM: u32 = 1;
pub const XLTYPE_STR: u32 = 2;
pub const XLTYPE_BOOL: u32 = 4;
pub const XLTYPE_REF: u32 = 8;
pub const XLTYPE_ERR: u32 = 16;
pub const XLTYPE_FLOW: u32 = 32;
pub const XLTYPE_MULTI: u32 = 64;
pub const XLTYPE_MISSING: u32 = 128;
pub const XLTYPE_NIL: u32 = 256;
pub const XLTYPE_SREF: u32 = 1024;
pub const XLTYPE_INT: u32 = 2048;

// free bit flag, shouldn't be needed
const XLBIT_XL_FREE: u32 = 4096;
const XLBIT_DLL_FREE: u32 = 16384;
const XLTYPE_BIG_DATA: u32 = 2050;

// error codes for XLOPER12
pub const XLERR_NULL: i32 = 0;
pub const XLERR_DIV0: i32 = 7;
pub const XLERR_VALUE: i32 = 15;
pub const XLERR_REF: i32 = 23;
pub const XLERR_NAME: i32 = 29;
pub const XLERR_NUM: i32 = 36;
pub const XLERR_NA: i32 = 42;
pub const XLERR_GETTING_DATA: i32 = 43;

// Excel12v function numbers
pub const XLF_REGISTER: i32 = 149;
pub const XL_FREE: i32 = 0x4000;
pub const XL_GET_NAME: i32 = 0x4000 | 9;

// Excel12v return codes
pub const XLRET_SUCCESS: i32 = 0;

// Mask off the xlbitXLFree / xlbitDLLFree flags to get the base type
pub const XLTYPE_MASK: u32 = 0x0FFF;

// typedefs from C, many unneeded

pub type WcharT = ::std::os::raw::c_ushort;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type INT32 = ::std::os::raw::c_int;
pub type UlongPtr = ::std::os::raw::c_ulonglong;
pub type DwordPtr = UlongPtr;
pub type CHAR = ::std::os::raw::c_char;
pub type LONG = ::std::os::raw::c_long;
pub type WCHAR = WcharT;
pub type LPSTR = *mut CHAR;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type XCHAR = WCHAR;
pub type RW = INT32;
pub type COL = INT32;
pub type IDSHEET = DwordPtr;

// xlref12 and xlmref12, definitions provided just to fit xloper12 struct shape

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct xlref12 {
    pub rw_first: RW,
    pub rw_last: RW,
    pub col_first: COL,
    pub col_last: COL,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xlref12"][::std::mem::size_of::<xlref12>() - 16usize];
    ["Alignment of xlref12"][::std::mem::align_of::<xlref12>() - 4usize];
    ["Offset of field: xlref12::rw_first"][::std::mem::offset_of!(xlref12, rw_first) - 0usize];
    ["Offset of field: xlref12::rw_last"][::std::mem::offset_of!(xlref12, rw_last) - 4usize];
    ["Offset of field: xlref12::col_first"][::std::mem::offset_of!(xlref12, col_first) - 8usize];
    ["Offset of field: xlref12::col_last"][::std::mem::offset_of!(xlref12, col_last) - 12usize];
};
pub type XLREF12 = xlref12;
pub type LPXLREF12 = *mut xlref12;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct xlmref12 {
    pub count: WORD,
    pub reftbl: [XLREF12; 1usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of xlmref12"][::std::mem::size_of::<xlmref12>() - 20usize];
    ["Alignment of xlmref12"][::std::mem::align_of::<xlmref12>() - 4usize];
    ["Offset of field: xlmref12::count"][::std::mem::offset_of!(xlmref12, count) - 0usize];
    ["Offset of field: xlmref12::reftbl"][::std::mem::offset_of!(xlmref12, reftbl) - 4usize];
};
pub type XLMREF12 = xlmref12;
pub type LPXLMREF12 = *mut xlmref12;

// not including FP12 for now, think it's better to run everything through XLOPER12 for v1

// XLOPER12
// TODO: Fix public / private access modifiers, bindgen pub by default

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xloper12 {
    pub val: Xloper12Val,
    pub xltype: DWORD,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Xloper12Val {
    pub num: f64,
    pub str_: *mut XCHAR,
    pub xbool: BOOL,
    pub err: ::std::os::raw::c_int,
    pub w: ::std::os::raw::c_int,
    pub sref: Xloper12SRef,
    pub mref: Xloper12MRef,
    pub array: Xloper12Array,
    pub flow: Xloper12Flow,
    pub bigdata: Xloper12BigData,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Xloper12SRef {
    pub count: WORD,
    pub ref_: XLREF12,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Xloper12SRef"]
        [::std::mem::size_of::<Xloper12SRef>() - 20usize];
    ["Alignment of Xloper12SRef"]
        [::std::mem::align_of::<Xloper12SRef>() - 4usize];
    ["Offset of field: Xloper12SRef::count"]
        [::std::mem::offset_of!(Xloper12SRef, count) - 0usize];
    ["Offset of field: Xloper12SRef::ref_"]
        [::std::mem::offset_of!(Xloper12SRef, ref_) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Xloper12MRef {
    pub lpmref: *mut XLMREF12,
    pub id_sheet: IDSHEET,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Xloper12MRef"]
        [::std::mem::size_of::<Xloper12MRef>() - 16usize];
    ["Alignment of Xloper12MRef"]
        [::std::mem::align_of::<Xloper12MRef>() - 8usize];
    ["Offset of field: Xloper12MRef::lpmref"]
        [::std::mem::offset_of!(Xloper12MRef, lpmref) - 0usize];
    ["Offset of field: Xloper12MRef::id_sheet"]
        [::std::mem::offset_of!(Xloper12MRef, id_sheet) - 8usize];
};
impl Default for Xloper12MRef {
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
pub struct Xloper12Array {
    pub lparray: *mut Xloper12,
    pub rows: RW,
    pub columns: COL,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Xloper12Array"]
        [::std::mem::size_of::<Xloper12Array>() - 16usize];
    ["Alignment of Xloper12Array"]
        [::std::mem::align_of::<Xloper12Array>() - 8usize];
    ["Offset of field: Xloper12Array::lparray"]
        [::std::mem::offset_of!(Xloper12Array, lparray) - 0usize];
    ["Offset of field: Xloper12Array::rows"]
        [::std::mem::offset_of!(Xloper12Array, rows) - 8usize];
    ["Offset of field: Xloper12Array::columns"]
        [::std::mem::offset_of!(Xloper12Array, columns) - 12usize];
};
impl Default for Xloper12Array {
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
pub struct Xloper12Flow {
    pub valflow: Xloper12FlowVal,
    pub rw: RW,
    pub col: COL,
    pub xlflow: BYTE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Xloper12FlowVal {
    pub level: ::std::os::raw::c_int,
    pub tbctrl: ::std::os::raw::c_int,
    pub id_sheet: IDSHEET,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Xloper12FlowVal"]
        [::std::mem::size_of::<Xloper12FlowVal>() - 8usize];
    ["Alignment of Xloper12FlowVal"]
        [::std::mem::align_of::<Xloper12FlowVal>() - 8usize];
    ["Offset of field: Xloper12FlowVal::level"][::std::mem::offset_of!(
        Xloper12FlowVal,
        level
    ) - 0usize];
    ["Offset of field: Xloper12FlowVal::tbctrl"][::std::mem::offset_of!(
        Xloper12FlowVal,
        tbctrl
    ) - 0usize];
    ["Offset of field: Xloper12FlowVal::id_sheet"][::std::mem::offset_of!(
        Xloper12FlowVal,
        id_sheet
    ) - 0usize];
};
impl Default for Xloper12FlowVal {
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
    ["Size of Xloper12Flow"]
        [::std::mem::size_of::<Xloper12Flow>() - 24usize];
    ["Alignment of Xloper12Flow"]
        [::std::mem::align_of::<Xloper12Flow>() - 8usize];
    ["Offset of field: Xloper12Flow::valflow"]
        [::std::mem::offset_of!(Xloper12Flow, valflow) - 0usize];
    ["Offset of field: Xloper12Flow::rw"]
        [::std::mem::offset_of!(Xloper12Flow, rw) - 8usize];
    ["Offset of field: Xloper12Flow::col"]
        [::std::mem::offset_of!(Xloper12Flow, col) - 12usize];
    ["Offset of field: Xloper12Flow::xlflow"]
        [::std::mem::offset_of!(Xloper12Flow, xlflow) - 16usize];
};
impl Default for Xloper12Flow {
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
pub struct Xloper12BigData {
    pub h: Xloper12BigDataHandle,
    pub cb_data: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Xloper12BigDataHandle {
    pub lpb_data: *mut BYTE,
    pub hdata: HANDLE,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Xloper12BigDataHandle"]
        [::std::mem::size_of::<Xloper12BigDataHandle>() - 8usize];
    ["Alignment of Xloper12BigDataHandle"]
        [::std::mem::align_of::<Xloper12BigDataHandle>() - 8usize];
    ["Offset of field: Xloper12BigDataHandle::lpb_data"][::std::mem::offset_of!(
        Xloper12BigDataHandle,
        lpb_data
    ) - 0usize];
    ["Offset of field: Xloper12BigDataHandle::hdata"][::std::mem::offset_of!(
        Xloper12BigDataHandle,
        hdata
    ) - 0usize];
};
impl Default for Xloper12BigDataHandle {
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
    ["Size of Xloper12BigData"]
        [::std::mem::size_of::<Xloper12BigData>() - 16usize];
    ["Alignment of Xloper12BigData"]
        [::std::mem::align_of::<Xloper12BigData>() - 8usize];
    ["Offset of field: Xloper12BigData::h"]
        [::std::mem::offset_of!(Xloper12BigData, h) - 0usize];
    ["Offset of field: Xloper12BigData::cb_data"]
        [::std::mem::offset_of!(Xloper12BigData, cb_data) - 8usize];
};
impl Default for Xloper12BigData {
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
    ["Size of Xloper12Val"][::std::mem::size_of::<Xloper12Val>() - 24usize];
    ["Alignment of Xloper12Val"]
        [::std::mem::align_of::<Xloper12Val>() - 8usize];
    ["Offset of field: Xloper12Val::num"]
        [::std::mem::offset_of!(Xloper12Val, num) - 0usize];
    ["Offset of field: Xloper12Val::str_"]
        [::std::mem::offset_of!(Xloper12Val, str_) - 0usize];
    ["Offset of field: Xloper12Val::xbool"]
        [::std::mem::offset_of!(Xloper12Val, xbool) - 0usize];
    ["Offset of field: Xloper12Val::err"]
        [::std::mem::offset_of!(Xloper12Val, err) - 0usize];
    ["Offset of field: Xloper12Val::w"]
        [::std::mem::offset_of!(Xloper12Val, w) - 0usize];
    ["Offset of field: Xloper12Val::sref"]
        [::std::mem::offset_of!(Xloper12Val, sref) - 0usize];
    ["Offset of field: Xloper12Val::mref"]
        [::std::mem::offset_of!(Xloper12Val, mref) - 0usize];
    ["Offset of field: Xloper12Val::array"]
        [::std::mem::offset_of!(Xloper12Val, array) - 0usize];
    ["Offset of field: Xloper12Val::flow"]
        [::std::mem::offset_of!(Xloper12Val, flow) - 0usize];
    ["Offset of field: Xloper12Val::bigdata"]
        [::std::mem::offset_of!(Xloper12Val, bigdata) - 0usize];
};
impl Default for Xloper12Val {
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
    ["Size of Xloper12"][::std::mem::size_of::<Xloper12>() - 32usize];
    ["Alignment of Xloper12"][::std::mem::align_of::<Xloper12>() - 8usize];
    ["Offset of field: Xloper12::val"][::std::mem::offset_of!(Xloper12, val) - 0usize];
    ["Offset of field: Xloper12::xltype"][::std::mem::offset_of!(Xloper12, xltype) - 24usize];
};
impl Default for Xloper12 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type XLOPER12 = Xloper12;
pub type LPXLOPER12 = *mut Xloper12;

// Excel12v callback, only one we'll need since Rust doesn't support varargs without macros
//
// The SDK's XLCALL32.LIB has no usable symbols, the real Excel12v is a shim in XLCALL.CPP that
// looks up Excel's exported "MdCallBack12" in the host process. Same thing here.

use std::ffi::{c_char, c_int, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};

const XLRET_FAILED: i32 = 32;
const XLRET_INV_COUNT: i32 = 4;
const MAX_OPERS: c_int = 255;

type Excel12Proc = unsafe extern "system" fn(
    xlfn: c_int,
    coper: c_int,
    rgpxloper12: *mut LPXLOPER12,
    oper_res: LPXLOPER12,
) -> c_int;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(module_name: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, proc_name: *const c_char) -> *mut c_void;
}

// 0 = not looked up yet / not found
static EXCEL12_ENTRY: AtomicUsize = AtomicUsize::new(0);

fn fetch_excel12_entry() -> Option<Excel12Proc> {
    let mut addr = EXCEL12_ENTRY.load(Ordering::Acquire);
    if addr == 0 {
        // Null module name = the host process, which is Excel.exe when loaded as an XLL
        let module = unsafe { GetModuleHandleW(std::ptr::null()) };
        if module.is_null() {
            return None;
        }
        addr = unsafe { GetProcAddress(module, c"MdCallBack12".as_ptr()) } as usize;
        EXCEL12_ENTRY.store(addr, Ordering::Release);
    }
    (addr != 0).then(|| unsafe { std::mem::transmute::<usize, Excel12Proc>(addr) })
}

/// # Safety
/// `opers` must point to `count` valid `LPXLOPER12`s, and `oper_res` must be null or valid.
pub unsafe fn excel12v(
    xlfn: c_int,
    oper_res: LPXLOPER12,
    count: c_int,
    opers: *mut LPXLOPER12,
) -> c_int {
    if !(0..=MAX_OPERS).contains(&count) {
        return XLRET_INV_COUNT;
    }
    match fetch_excel12_entry() {
        Some(proc) => unsafe { proc(xlfn, count, opers, oper_res) },
        None => XLRET_FAILED,
    }
}
