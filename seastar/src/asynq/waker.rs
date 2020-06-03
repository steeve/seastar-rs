use std::task::{RawWaker, RawWakerVTable, Waker};
use std::ptr;
use std::task::Context;
use super::executor::Task;
use std::task::Poll;

type WakerData = *const ();

unsafe fn clone(_: WakerData) -> RawWaker {
    noop_raw_waker()
}
unsafe fn wake(_: WakerData) {}
unsafe fn wake_by_ref(_: WakerData) {}
unsafe fn drop(_: WakerData) {}

static MY_VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone,
    wake,
    wake_by_ref,
    drop
);

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(ptr::null(), &MY_VTABLE)
}

pub fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

static SEASTAR_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    SeastarWaker::clone,
    SeastarWaker::wake,
    SeastarWaker::wake_by_ref,
    SeastarWaker::drop,
);

pub struct SeastarWaker {
}

impl SeastarWaker {
    unsafe fn wake(data: *const ()) {
        // let mut task = (data as *mut Task);
        // (*waker).poll();
    }

    unsafe fn wake_by_ref(data: *const ()) {
        let mut task = data as *mut Task;
        let waker = Waker::from_raw(Self::clone(data));
        let mut context = Context::from_waker(&waker);
        if (*task).future.as_mut().poll(&mut context) == Poll::Ready(()) {
            Box::from_raw(task);
        }
    }

    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &SEASTAR_WAKER_VTABLE)
    }

    unsafe fn drop(data: *const ()) {
    }

    pub fn from_task(task: Task) -> Waker {
        unsafe {
            let data = Box::into_raw(Box::new(task)) as *const Task as *const ();
            Waker::from_raw(Self::clone(data))
        }
    }
}
