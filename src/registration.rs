use crate::marshalling::xlcall::{
    excel12v, LPXLOPER12, XLF_REGISTER, XLOPER12, XLRET_SUCCESS, XLTYPE_MISSING, XLTYPE_NUM,
    XLTYPE_STR, XL_FREE, XL_GET_NAME,
};

// Excel caps most registration strings at 255 characters
const MAX_REG_STR_LEN: usize = 255;

// Excel wants counted UTF-16 strings: first u16 is the length, no null terminator
fn to_pascal_wide(s: &str) -> Vec<u16> {
    let units: Vec<u16> = s.encode_utf16().take(MAX_REG_STR_LEN).collect();
    let mut buf = Vec::with_capacity(units.len() + 1);
    buf.push(units.len() as u16);
    buf.extend(units);
    buf
}

// `buf` must outlive any call that receives the returned XLOPER12
fn str_oper(buf: &mut [u16]) -> XLOPER12 {
    let mut oper = XLOPER12::default();
    oper.xltype = XLTYPE_STR;
    oper.val.str_ = buf.as_mut_ptr();
    oper
}

fn num_oper(n: f64) -> XLOPER12 {
    let mut oper = XLOPER12::default();
    oper.xltype = XLTYPE_NUM;
    oper.val.num = n;
    oper
}

fn missing_oper() -> XLOPER12 {
    let mut oper = XLOPER12::default();
    oper.xltype = XLTYPE_MISSING;
    oper
}

/// Registers one worksheet function with Excel via xlfRegister.
///
/// `meta` is, in order: exported procedure name, type string (e.g. "QQQ"), Excel function name,
/// argument names ("a, b"), function description, then one description per argument.
/// Returns excel12v's return code, 0 on success.
pub fn register_function(meta: &[&str]) -> i32 {
    // Procedure, type text and function name are required
    if meta.len() < 3 {
        return -1;
    }

    // Registration strings, kept alive until after the excel12v call
    let mut bufs: Vec<Vec<u16>> = meta.iter().map(|s| to_pascal_wide(s)).collect();
    let mut category = to_pascal_wide("Rusty UDF");
    let mut empty = to_pascal_wide("");

    // Ask Excel for the path of this XLL, which is the first xlfRegister arg
    let mut dll = XLOPER12::default();
    let ret = unsafe { excel12v(XL_GET_NAME, &mut dll, 0, std::ptr::null_mut()) };
    if ret != XLRET_SUCCESS {
        return ret;
    }

    // xlfRegister arg order: dll, procedure, type text, function name, argument names,
    // macro type, category, shortcut, help topic, function help, argument help...
    let mut opers: Vec<XLOPER12> = Vec::with_capacity(meta.len() + 7);
    opers.push(dll);
    opers.push(str_oper(&mut bufs[0]));
    opers.push(str_oper(&mut bufs[1]));
    opers.push(str_oper(&mut bufs[2]));

    // Argument names, empty string if not given
    match bufs.get_mut(3) {
        Some(buf) => opers.push(str_oper(buf)),
        None => opers.push(str_oper(&mut empty)),
    }

    opers.push(num_oper(1.0)); // macro type 1 = worksheet function
    opers.push(str_oper(&mut category));
    opers.push(missing_oper()); // shortcut
    opers.push(missing_oper()); // help topic

    // Function help and per-argument help, if given
    for buf in bufs.iter_mut().skip(4) {
        opers.push(str_oper(buf));
    }

    // Excel wants an array of pointers rather than a pointer to an array of XLOPER12
    let mut ptrs: Vec<LPXLOPER12> = opers.iter_mut().map(|o| o as *mut XLOPER12).collect();
    let ret = unsafe {
        excel12v(XLF_REGISTER, std::ptr::null_mut(), ptrs.len() as i32, ptrs.as_mut_ptr())
    };

    // The xlGetName result is Excel-allocated and has to be handed back
    let mut free_arg: LPXLOPER12 = &mut opers[0];
    unsafe { excel12v(XL_FREE, std::ptr::null_mut(), 1, &mut free_arg) };

    ret
}
