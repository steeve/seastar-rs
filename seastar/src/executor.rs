extern crate libc;
use libc::c_void;

// mod closures;

extern "C" {
    fn seastar_spawn(ptr: *mut c_void);
}

fn spawn(f: impl FnOnce() + 'static) {
    let ptr = closure_to_ptr!(f);
    unsafe { seastar_spawn(ptr) };
}
