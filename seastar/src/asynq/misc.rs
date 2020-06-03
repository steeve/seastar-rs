use std::time::Duration;
use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::cell::RefCell;
use std::task::Waker;

struct FutureState<T> {
    result: Poll<T>,
    waker: Option<Waker>,
}

pub struct SeastarFutureBase<FunctionType, ReturnType> {
    state: Rc<UnsafeCell<FutureState<ReturnType>>>,
    f: FunctionType,
}

impl<FunctionType, ReturnType> SeastarFutureBase<FunctionType, ReturnType>
where
    FunctionType: Fn(Box<dyn Fn(ReturnType)>) + 'static,
    ReturnType: Copy,
{
    pub fn new(f: FunctionType) -> SeastarFutureBase<FunctionType, ReturnType> {
        SeastarFutureBase::<FunctionType, ReturnType> {
            state: Rc::new(UnsafeCell::new(FutureState {
                result: Poll::Pending,
                waker: None,
            })),
            f: f,
        }
    }
}

impl<FunctionType, ReturnType> std::future::Future for SeastarFutureBase<FunctionType, ReturnType>
where
    FunctionType: Fn(Box<dyn Fn(ReturnType)>) + 'static,
    ReturnType: Copy + 'static,
{
    type Output = ReturnType;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut state = unsafe { &mut *self.state.get() };
        let waker = Some(cx.waker().clone());
        match state.waker {
            None => {
                state.waker = waker;
                let nstate = self.state.clone();
                (self.f)(Box::new(move |result| {
                    let mut state = unsafe { &mut *nstate.get() };
                    state.result = Poll::Ready(result);
                    state.waker.as_ref().unwrap().wake_by_ref();
                }));
            }
            Some(_) => {
                state.waker = waker;
            }
        };
        state.result
    }
}

pub fn sleep(duration: Duration) -> SeastarFutureBase<impl Fn(Box<dyn Fn(())>), ()>
{
    SeastarFutureBase::new(move |done| {
        super::super::sleep(duration, move || done(()));
    })
}

pub fn file_size(file: String) -> SeastarFutureBase<impl Fn(Box<dyn Fn(u64)>), u64> {
    SeastarFutureBase::new(move |done| {
        super::super::file::size(&file, done);
    })
}


// pub struct SleepFuture {
//     state: Rc<UnsafeCell<FutureState<()>>>,
//     duration: Duration,
// }

// impl std::future::Future for SleepFuture {
//     type Output = ();

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
//         let mut state = unsafe { &mut *self.state.get() };
//         if state.waker.is_none() {
//             state.waker = Some(cx.waker().clone());
//             let nstate = self.state.clone();
//             super::super::sleep(self.duration, move || {
//                 let mut state = unsafe { &mut *nstate.get() };
//                 state.result = Poll::Ready(());
//                 if let Some(waker) = &state.waker {
//                     waker.wake_by_ref();
//                 }
//             });
//         } else {
//             state.waker = Some(cx.waker().clone());
//         }
//         state.result
//     }
// }

// pub fn sleep(duration: Duration) -> SleepFuture {
//     SleepFuture {
//         state: Rc::new(UnsafeCell::new(FutureState {
//             result: Poll::Pending,
//             waker: None,
//         })),
//         duration: duration,
//     }
// }

// pub struct FileSizeFuture {
//     state: Rc<UnsafeCell<FutureState<u64>>>,
//     file: String,
// }

// impl std::future::Future for FileSizeFuture {
//     type Output = u64;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
//         let mut state = unsafe { &mut *self.state.get() };
//         if state.waker.is_none() {
//             state.waker = Some(cx.waker().clone());
//             let nstate = self.state.clone();
//             super::super::file::size(&self.file, move |size| {
//                 let mut state = unsafe { &mut *nstate.get() };
//                 state.result = Poll::Ready(size);
//                 if let Some(waker) = &state.waker {
//                     waker.wake_by_ref();
//                 }
//             });
//         } else {
//             state.waker = Some(cx.waker().clone());
//         }
//         state.result
//     }
// }

// pub fn file_size(file: String) -> FileSizeFuture {
//     FileSizeFuture {
//         state: Rc::new(UnsafeCell::new(FutureState {
//             result: Poll::Pending,
//             waker: None,
//         })),
//         file: file,
//     }
// }
