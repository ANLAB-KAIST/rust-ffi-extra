//! Useful tools for C FFI

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr;

/// Call a main-like function with an argument vector
///
/// # Safety
///
/// Safety depends on the safety of the target FFI function.
pub unsafe fn run_with_args<
    'a,
    StringLike: 'a + Clone + Into<Vec<u8>>,
    Argv: IntoIterator<Item = &'a StringLike>,
>(
    func: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int,
    args: Argv,
) -> i32 {
    let mut cstring_buffer: Vec<CString> = Vec::new();

    // 1. First clone the string values into safe storage.
    for arg in args {
        cstring_buffer
            .push(CString::new(arg.clone()).expect("String to CString conversion failed"));
    }

    // 2. Total number of args is fixed.
    let argc = cstring_buffer.len() as c_int;

    // 3. Prepare raw vector
    let mut c_char_buffer: Vec<*mut c_char> = Vec::new();
    for cstring in &cstring_buffer {
        c_char_buffer.push(cstring.as_bytes_with_nul().as_ptr() as *mut c_char);
    }
    c_char_buffer.push(ptr::null_mut());

    let c_argv = c_char_buffer.as_mut_ptr();

    // 4. Now call the function
    func(argc, c_argv) as i32
}
