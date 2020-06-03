#[macro_export]
macro_rules! closure_to_ptr {
    ($closure:expr) => {
        {
            use std::boxed::Box;
            use std::os::raw::c_void;
            let closure = $closure;
            type ClosureType = Box<Box<dyn FnOnce() + 'static>>;
            Box::into_raw(ClosureType::new(Box::new(closure))) as *mut c_void
        }
    };
    ($type:ty, $closure:expr) => {
        {
            use std::boxed::Box;
            use std::os::raw::c_void;
            let closure = $closure;
            type ClosureType = Box<Box<dyn FnOnce($type) + 'static>>;
            Box::into_raw(ClosureType::new(Box::new(closure))) as *mut c_void
        }
    };
}

// extern crate libc;
// use libc::c_void;
// use libc::c_char;

// macro_rules! closure_generate {
//     ($from_name:ident) => {
//         #[no_mangle]
//         pub extern "C" fn $from_name(ptr: *mut c_void) {
//             type ClosureType = Box<Box<dyn FnOnce() + 'static>>;
//             let cbox = unsafe { ClosureType::from_raw(ptr as *mut _) };
//             (*cbox)();
//         }
//     };
//     ($from_name:ident, $type:ty) => {
//         #[no_mangle]
//         pub extern "C" fn $from_name(ptr: *mut c_void, v: $type) {
//             type ClosureType = Box<Box<dyn FnOnce($type) + 'static>>;
//             let cbox = unsafe { ClosureType::from_raw(ptr as *mut _) };
//             (*cbox)(v);
//         }
//     };
// }

// closure_generate!(seastar_rs_closure_void);
// closure_generate!(seastar_rs_closure_bool, bool);
// closure_generate!(seastar_rs_closure_i32, i32);
// closure_generate!(seastar_rs_closure_i64, i64);
// closure_generate!(seastar_rs_closure_u32, u32);
// closure_generate!(seastar_rs_closure_u64, u64);
// closure_generate!(seastar_rs_closure_usize, usize);
// closure_generate!(seastar_rs_closure_voidptr, *mut c_void);

// extern "C" {
//     fn seastar_sleep(ns: u64, ptr: *mut c_void);
//     // fn seastar_spawn(ptr: *mut c_void);
//     fn seastar_file_size(name: *const c_char, ptr: *mut c_void);
// }

// fn sleep(duration: Duration, f: impl FnOnce() + 'static) {
//     let ptr = closure_to_ptr!(f);
//     unsafe { seastar_sleep(duration.as_nanos() as u64, ptr) };
// }

// fn spawn(f: impl FnOnce() + 'static) {
//     let ptr = closure_to_ptr!(f);
//     unsafe { seastar_spawn(ptr) };
// }

// fn file_size(name: &str, f: impl FnOnce(u64) + 'static) {
//     use std::ffi::CString;
//     let str = CString::new(name).unwrap();
//     let ptr = closure_to_ptr!(u64, f);
//     unsafe { seastar_file_size(str.as_ptr(), ptr) };
// }

// pub struct SleepFuture {
//     started: bool,
// }

// impl std::future::Future for SleepFuture {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
//         let task = current_task.with(|t| *t.borrow() );
//         if self.started == false {
//             self.started = true;
//             unsafe { seastar_make_sleep_future(task, 1e9 as u64) };
//             return Poll::Pending;
//         }
//         Poll::Ready(())
//     }
// }

// pub fn sleep() -> SleepFuture {
//     SleepFuture {
//         started: false,
//     }
// }
