#![crate_name = "seastar"]
#![crate_type = "staticlib"]

#[macro_use] mod closures;
pub mod app_template;

pub mod misc;
pub use self::misc::*;

pub mod file;

pub mod asynq;

// #[macro_use]
// mod closures;

// mod executor;

// mod file;

// mod main;

// extern crate libc;
// use libc::{c_void};

// #[macro_use]
// extern crate lazy_static;

// use std::{
//     time::Duration,
//     future::Future,
//     ptr,
//     pin::Pin,
//     task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
//     sync::{Arc, Mutex},
// };

// #[no_mangle]
// pub extern "C" fn seastar_rs_future_poll(fptr: *mut c_void) {
//     // let mut fbox = unsafe { Seastar::BoxedFuture::from_raw(fptr as *mut _) };

//     // let waker = my_waker();
//     // println!("{}:{}", file!(), line!());
//     // let mut context = Context::from_waker(&waker);
//     // println!("{}:{}", file!(), line!());
//     // (*fbox).as_mut().poll(&mut context);
//     // println!("{}:{}", file!(), line!());
// }

// async fn coucou() {
//     dbg!("COUCOU 1");
//     Seastar::sleep().await;
//     dbg!("COUCOU 2");
// }

// #[no_mangle]
// pub extern "C" fn seastar_rs_main() -> *mut c_void {
//     Seastar::spawn(async move {
//         println!("BEGIN");
//         Seastar::sleep().await;
//         let one = coucou();
//         let two = coucou();
//         one.await;
//         two.await;
//         println!("END");
//     });
//     // let fut = Seastar::sleep(Duration::from_secs(1));
//     // Seastar::future_to_ptr(seastar_async_main())
//     std::ptr::null_mut()
// }

// mod Seastar {
//     use std::future::Future;
//     use std::task::Poll;
//     use std::task::Context;
//     use std::pin::Pin;
//     use std::boxed::Box;
//     use std::cell::{UnsafeCell, RefCell};
//     use std::time::Duration;
//     use std::task::{RawWaker, RawWakerVTable, Waker};
//     use std::ptr;
//     use std::thread;
//     use std::option::Option;
//     use std::option::Option::{Some, None};
//     extern crate libc;
//     use libc::{c_void};

//     type WakerData = *const ();

//     thread_local! {
//         static current_task: RefCell<*mut Task> = RefCell::new(std::ptr::null_mut());
//     }

//     unsafe fn clone(_: WakerData) -> RawWaker {
//         my_raw_waker()
//     }
//     unsafe fn wake(_: WakerData) {}
//     unsafe fn wake_by_ref(_: WakerData) {}
//     unsafe fn drop(_: WakerData) {}

//     static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

//     fn my_raw_waker() -> RawWaker {
//         RawWaker::new(ptr::null(), &MY_VTABLE)
//     }

//     fn my_waker() -> Waker {
//         unsafe { Waker::from_raw(my_raw_waker()) }
//     }

//     static SEASTAR_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
//         sclone,
//         swake,
//         swake_by_ref,
//         drop,
//     );

//     pub struct SWakerData {
//         task: Task,
//     }

//     unsafe fn swake(data: *const ()) {
//         let mut waker = (data as *mut Task);
//         (*waker).poll();
//     }

//     unsafe fn swake_by_ref(data: *const ()) {
//         dbg!("IN WAKER");
//         let mut waker = (data as *mut Task);
//         dbg!("IN WAKER 2");
//         dbg!((*waker).value);
//         // (*waker).poll();
//     }

//     unsafe fn sclone(data: *const ()) -> RawWaker {
//         RawWaker::new(data, &SEASTAR_WAKER_VTABLE)
//     }

//     fn raw_swaker(task: Task) -> RawWaker {
//         unsafe {
//             sclone(Box::into_raw(Box::new(task)) as *const Task as *const ())
//         }
//     }

//     fn swaker(task: Task) -> Waker {
//         unsafe { Waker::from_raw(raw_swaker(task)) }
//     }

//     extern "C" {
//         fn seastar_spawn(task: *mut Task);
//         fn seastar_make_sleep_future(f: *mut Task, ns: u64);
//         fn is_future_ready(fptr: *mut c_void) -> usize;
//     }

//     type BoxedFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
//     type BoxedTask = Box<Task>;

//     #[repr(C)]
//     pub struct Task {
//         value: usize,
//         future: BoxedFuture,
//     }

//     #[no_mangle]
//     pub extern "C" fn seastar_rs_task_complete(task: *mut Task) {
//         dbg!("TASK COMPLETE");
//         unsafe {
//             (*task).poll();
//         }
//     }

//     #[no_mangle]
//     pub extern "C" fn seastar_rs_task_poll(task: *mut Task) {
//         current_task.with(|t| {
//             *t.borrow_mut() = task;
//         });
//         unsafe {
//             (*task).poll();
//         };
//     }

//     impl Task {
//         fn poll(&mut self) {
//             let waker = my_waker();
//             let mut context = Context::from_waker(&waker);
//             self.future.as_mut().poll(&mut context);
//         }
//     }

//     pub fn spawn(f: impl Future<Output = ()> + 'static) {
//         let mut task = Task {
//             value: 123,
//             future: Box::pin(f),
//         };
//         unsafe { seastar_spawn(Box::into_raw(Box::new(task))) };
//     }

//     pub struct FutureState {
//         started: usize,
//         completed: usize,
//     }

//     pub struct SleepFuture {
//         started: bool,
//     }

//     impl std::future::Future for SleepFuture {
//         type Output = ();

//         fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
//             let task = current_task.with(|t| *t.borrow() );
//             if self.started == false {
//                 self.started = true;
//                 unsafe { seastar_make_sleep_future(task, 1e9 as u64) };
//                 return Poll::Pending;
//             }
//             Poll::Ready(())
//         }
//     }

//     pub fn sleep() -> SleepFuture {
//         SleepFuture {
//             started: false,
//         }
//     }

// }

//     // pub struct SleepFuture {
//     //     state: Pin<Box<_FutureState>>,
//     // }

//     // impl std::future::Future for SleepFuture {
//     //     type Output = ();

//     //     fn poll<'a>(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
//     //         if self.state.started == 0 {
//     //         //     self.state.started = 1;
//     //         //     seastar_make_sleep_future(ns.as_nanos() as u64, Box::into_raw(Box::new(self.state)));
//     //             let fptr = future_to_ptr(self);
//     //             unsafe { seastar_make_sleep_future(1e9 as u64, &mut (*self.state), fptr) };
//     //         }
//     //         println!("COUCOU {}", (*self.state).completed);
//     //         match (*self.state).completed {
//     //             0 => Poll::Pending,
//     //             _ => Poll::Ready(()),
//     //         }
//     //     }
//     // }

//     // pub fn sleep(duration: std::time::Duration) -> SleepFuture {
//     //     let mut fut = SleepFuture {
//     //         state: Box::pin(_FutureState {
//     //             started: 0,
//     //             completed: 0,
//     //         }),
//     //     };
//     //     fut
//     // }

//     // fn future_to_ptr(f: impl Future<Output = ()> + 'static) -> *mut c_void {
//     //     let fptr = BoxedFuture::new(Box::pin(f));
//     //     Box::into_raw(fptr) as *mut c_void
//     // }

//     // pub fn spawn(f: impl Future<Output = ()> + 'static) {
//     //     let fptr = future_to_ptr(f);
//     //     unsafe { seastar_spawn(fptr) };
//     // }
// // }

// // type BoxedFuture = Box<Pin<Box<dyn Future<Output = ()> + 'static>>>;
// // type BoxedClosure = Box<Box<dyn FnMut()>>;

// // type WakerData = *const ();

// // unsafe fn clone(_: WakerData) -> RawWaker {
// //     my_raw_waker()
// // }
// // unsafe fn wake(_: WakerData) {}
// // unsafe fn wake_by_ref(_: WakerData) {}
// // unsafe fn drop(_: WakerData) {}

// // static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

// // fn my_raw_waker() -> RawWaker {
// //     RawWaker::new(ptr::null(), &MY_VTABLE)
// // }

// // fn my_waker() -> Waker {
// //     unsafe { Waker::from_raw(my_raw_waker()) }
// // }

// // async fn hello() -> i32 {
// //     println!("Hello, World!");
// //     1
// // }

// // async fn hello2() {
// //     println!("Hello, World 2!");
// //     hello().await;
// //     hello().await;
// //     println!("Hello, World 3!");
// // }

// // // extern "C" {
// // //     fn seastar_spawn(fptr: *mut c_void);
// // //     fn seastar_make_sleep_future(ns: u64) -> *mut c_void;
// // //     fn is_future_ready(fptr: *mut c_void) -> usize;
// // // }

// // async fn seastar_async_main() {
// //     println!("1");
// //     // Seastar::sleep(Duration::from_secs(1)).await;
// //     println!("2");
// //     // Seastar::sleep(Duration::from_secs(1)).await;
// //     println!("3");
// // }

// // #[derive(Debug)]
// // pub struct SeastarSleepFuture {
// //     fptr: *mut c_void,
// //     waker: Option<Waker>,
// // }

// // impl Future for SeastarSleepFuture {
// //     type Output = ();

// //     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
// //         println!(">>>>POLL {}", 0);
// //         let result = 1;
// //         match result {
// //             0 => Poll::Pending,
// //             _ => Poll::Ready(()),
// //         }
// //     }
// // }

// // impl SeastarSleepFuture {
// //     pub fn new(ns: Duration) -> Self {
// //         SeastarSleepFuture {
// //             fptr: unsafe { seastar_make_sleep_future(ns.as_nanos() as u64) },
// //             waker: None,
// //         }
// //     }
// // }

//     // fn ptr_to_future(fptr: *mut c_void) -> PinFuture {
//     //     let obox = unsafe { BoxedFuture::from_raw(fptr as *mut _) };
//     //     *obox
//     // }

//     // fn closure_to_ptr(f: impl FnMut() + 'static) -> *mut c_void {
//     //     let fbox = BoxedClosure::new(Box::new(f));
//     //     Box::into_raw(fbox) as *mut c_void
//     // }

//     // fn ptr_to_closure(fptr: *mut c_void) -> Box<dyn FnMut()> {
//     //     let cbox = unsafe { BoxedClosure::from_raw(fptr as *mut _) };
//     //     *cbox
//     // }

//     // pub fn sleep(ns: Duration, f: impl FnMut() + 'static) {
//     //     let fptr = Seastar::closure_to_ptr(f);
//     //     unsafe { seastar_sleep(fptr, ns.as_nanos() as u64) };
//     // }

//     // pub fn sleep2(ns: Duration, f: impl FnMut() + 'static) {
//     //     let fptr = Seastar::closure_to_ptr(f);
//     //     unsafe { seastar_make_sleep_future(ns.as_nanos() as u64, fptr) };
//     // }

//     // pub fn resume_closure(fptr: *mut c_void) {
//     //     let mut cbox = unsafe { BoxedClosure::from_raw(fptr as *mut _) };
//     //     (**cbox)();
//     // }

//     // pub fn future_poll(fptr: *mut c_void) {
//         // println!("{}:{}", file!(), line!());
//         // let waker = my_waker();
//         // println!("{}:{}", file!(), line!());
//         // let mut context = Context::from_waker(&waker);
//         // println!("{}:{}", file!(), line!());
//         // let mut fut = Seastar::ptr_to_future(fptr);
//         // println!("{}:{}", file!(), line!());
//         // fut.as_mut().poll(&mut context);
//         // println!("{}:{}", file!(), line!());
//     // }

//     // pub fn resume_task(fptr: *mut c_void) {
//         // println!(">>> RESUME");
//         // Seastar::poll_future(fptr);
//         // let mut closure = Seastar::ptr_to_closure(fptr);
//         // (*closure)();
//         // let fbox: Box<FnMut()> = unsafe { Box::from_raw(fptr as *mut _) };
//         // fbox();
//         // let waker = my_waker();
//         // let mut context = Context::from_waker(&waker);

//         // let mut fut = Seastar::ptr_to_future(fptr);

//         // if let Poll::Pending = fut.as_mut().poll(&mut context) {
//         //     // panic!("SHOULD NOT POLL A PENDING TASK");
//         // }
//     // }
// // }

// // #[no_mangle]
// // pub extern "C" fn seastar_main() {
// //     // Seastar::sleep(1000000000)
// //     // println!("Hello from Rust");
// //     // let fptr = future_to_ptr(ss_main());
// //     // unsafe {
// //     //     seastar_enqueue(fptr);
// //     // }
// // }

// // fn future_to_ptr<F>(f: F) -> *mut c_void
// // where
// //     F: Future<Output = ()> + Send + 'static,
// // {
// //     let fbox = Box::pin(f);
// //     let fptr = BoxedFuture::new(fbox);
// //     Box::into_raw(fptr) as *mut c_void
// // }

// // fn ptr_to_future(fptr: *mut c_void) -> Pin<Box<dyn Future<Output = ()>>> {
// //     let obox = unsafe { BoxedFuture::from_raw(fptr as *mut _) };
// //     *obox
// // }

// // // fn main() {
// // //     let fptr = future_to_ptr(hello2());
// // //     unsafe {
// // //         seastar_sleep(fptr2, 1000000000);
// // //     }
// // // }

// // fn rs_poll<F>(f: F)
// // where
// //     F: Future,
// // {
// //     let waker = my_waker();
// //     let mut context = Context::from_waker(&waker);

// //     let mut t = Box::pin(f);
// //     let t = t.as_mut();

// //     t.poll(&mut context);
// // }
