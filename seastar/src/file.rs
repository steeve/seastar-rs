#[macro_use]
use super::closures::*;

extern crate seastar_sys;

pub fn size(name: &str, f: impl FnOnce(u64) + 'static) {
    use std::ffi::CString;
    let str = CString::new(name).unwrap();
    let ptr = closure_to_ptr!(u64, f);
    unsafe { seastar_sys::seastar_file_size(ptr, str.as_ptr()) };
}

// pub fn open_file_dma(name: &str, flags: usize, f: impl FnOnce(u64) + 'static) {
//     use std::ffi::CString;
//     let str = CString::new(name).unwrap();
//     let ptr = closure_to_ptr!(u64, f);
//     unsafe { seastar_sys::seastar_file_size(ptr, str.as_ptr()) };
// }
