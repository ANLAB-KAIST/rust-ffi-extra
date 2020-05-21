use ffi_extra::run_with_args;

use std::os::raw;
use std::ptr;

unsafe extern "C" fn validate_argv(argc: raw::c_int, argv: *mut *mut raw::c_char) -> i32 {
    assert!(argc < 36);
    for i in 0..argc {
        let character: char = *((*(argv.add(i as usize))).add(0)) as u8 as char;
        let end: u8 = *((*(argv.add(i as usize))).add(1)) as u8;
        assert_eq!(character.to_digit(36).unwrap(), i as u32);
        assert_eq!(end, 0);
    }
    assert_eq!(*argv.add(argc as usize), ptr::null_mut());
    return 0;
}

#[test]
fn test_argv_0() {
    let args: Vec<String> = vec![];
    unsafe { run_with_args(validate_argv, &args) };
}

#[test]
fn test_argv_1() {
    let args = vec!["0"];
    unsafe { run_with_args(validate_argv, &args) };
}

#[test]
fn test_argv_2() {
    let args = vec!["0", "1"];
    unsafe { run_with_args(validate_argv, &args) };
}
