extern crate seastar_sys;

use std::os::raw::c_int;
use std::os::raw::c_char;

use std::ffi::CString;

#[macro_use]
use super::closures::*;

pub fn run(f: impl FnOnce() + 'static) {
    let c_args: Vec<*const c_char> = std::env::args()
        .map(|arg| CString::new(arg).unwrap().as_ptr() )
        .collect();
    unsafe {
        seastar_sys::seastar_app_template_run(
            closure_to_ptr!(f),
            c_args.len() as c_int,
            c_args.as_ptr());
    };
}
