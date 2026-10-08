mod marshalling;
mod registration;

use marshalling::xlcall::{
    LPXLOPER12, XLERR_VALUE, XLOPER12, XLTYPE_ERR, XLTYPE_MASK, XLTYPE_NUM,
};
use std::cell::Cell;

// Testing only; first entry is the exported symbol name, "QQQ" is XLOPER12* return + two XLOPER12* args
static TEST_FN_STRS: &[&str] = &["xl_add", "QQQ", "ADD_BOTH", "a, b", "Adds two numbers", "1st number", "2nd number"];

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

thread_local! {
    // Excel reads the returned pointer after the call, so the result can't live on the stack
    static XL_RESULT: Cell<XLOPER12> = Cell::new(XLOPER12::default());
}

fn return_oper(oper: XLOPER12) -> LPXLOPER12 {
    XL_RESULT.with(|cell| {
        cell.set(oper);
        cell.as_ptr()
    })
}

// Testing only, exported so xlfRegister can find it by name
#[unsafe(no_mangle)]
pub extern "system" fn xl_add(a: LPXLOPER12, b: LPXLOPER12) -> LPXLOPER12 {
    let mut result = XLOPER12::default();

    let (a, b) = unsafe { (a.as_ref(), b.as_ref()) };
    match (a, b) {
        (Some(a), Some(b))
            if a.xltype & XLTYPE_MASK == XLTYPE_NUM && b.xltype & XLTYPE_MASK == XLTYPE_NUM =>
        {
            result.xltype = XLTYPE_NUM;
            result.val.num = unsafe { a.val.num + b.val.num };
        }
        // Anything that isn't a plain number comes back as #VALUE!
        _ => {
            result.xltype = XLTYPE_ERR;
            result.val.err = XLERR_VALUE;
        }
    }

    return_oper(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn xlAutoOpen() -> i32 {
    // Testing only, register function
    registration::register_function(TEST_FN_STRS);
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn xlAutoClose() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
