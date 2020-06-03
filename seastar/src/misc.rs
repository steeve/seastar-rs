use std::time::Duration;

use super::closures::*;

extern crate seastar_sys;

pub fn sleep(duration: Duration, f: impl FnOnce() + 'static) {
    let ptr = closure_to_ptr!(f);
    unsafe { seastar_sys::seastar_sleep(ptr, duration.as_nanos() as u64) };
}

pub fn spawn(f: impl FnOnce() + 'static) {
    let ptr = closure_to_ptr!(f);
    unsafe { seastar_sys::seastar_spawn(ptr) };
}
