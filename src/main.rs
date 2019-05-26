extern crate libc;

use std::env;
use std::ffi::CString;
use libc::{c_int, c_char};

extern "C" {
    fn main_c(argc: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    // Get the current arguments  and map them into a vector
    let args: Vec<CString> = env::args().filter_map(|arg| {
        CString::new(arg).ok()
    }).collect();

    // Convertir las CStrings a apuntadores
    let c_args: Vec<*const c_char> = args.iter().map(|arg| {
        arg.as_ptr()
    }).collect();

    // Call the main function within the created c library
    unsafe {
        main_c(c_args.len() as c_int, c_args.as_ptr());
    };
}
