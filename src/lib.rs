#![feature(libc)]

extern crate libc;
use std::ffi::CString;


#[no_mangle]
pub extern fn rust_text() -> *const libc::c_char
{
   let to_print = CString::new("Hello from Rust!\n").unwrap();

   to_print.as_ptr()
}
