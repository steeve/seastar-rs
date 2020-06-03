use std::os::raw::c_int;
use std::os::raw::c_void;
use std::os::raw::c_char;

macro_rules! closure_generate {
    ($from_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $from_name(ptr: *mut c_void) {
            type ClosureType = Box<Box<dyn FnOnce() + 'static>>;
            let cbox = unsafe { ClosureType::from_raw(ptr as *mut _) };
            (*cbox)();
        }
    };
    ($from_name:ident, $type:ty) => {
        #[no_mangle]
        pub extern "C" fn $from_name(ptr: *mut c_void, v: $type) {
            type ClosureType = Box<Box<dyn FnOnce($type) + 'static>>;
            let cbox = unsafe { ClosureType::from_raw(ptr as *mut _) };
            (*cbox)(v);
        }
    };
}

closure_generate!(seastar_rs_closure_void);
closure_generate!(seastar_rs_closure_bool, bool);
closure_generate!(seastar_rs_closure_i32, i32);
closure_generate!(seastar_rs_closure_i64, i64);
closure_generate!(seastar_rs_closure_u32, u32);
closure_generate!(seastar_rs_closure_u64, u64);
closure_generate!(seastar_rs_closure_usize, usize);
closure_generate!(seastar_rs_closure_voidptr, *mut c_void);

extern "C" {
    pub fn seastar_app_template_run(ctx: *mut c_void, argc: c_int, argv: *const *const c_char);
    pub fn seastar_sleep(ctx: *mut c_void, ns: u64);
    pub fn seastar_spawn(ctx: *mut c_void);
    pub fn seastar_file_size(ctx: *mut c_void, name: *const c_char);
}
