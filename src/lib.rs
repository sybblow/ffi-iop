#![allow(non_upper_case_globals)]

extern crate libc;
use std::ffi::CString;


#[no_mangle]
#[repr(C)]
pub static three: [u8; 3] = [78, 2, 3];

extern {
   fn cmax(num1: libc::c_uint, num2: libc::c_uint) -> libc::c_uint;
}

#[no_mangle]
pub extern fn rust_text() -> *const libc::c_char
{
   let max_value = unsafe {
      cmax(40, 80)
   };

   let s = format!("Hello from Rust - {}!", max_value);

   let to_print = CString::new(s).unwrap();

   to_print.as_ptr()
}
