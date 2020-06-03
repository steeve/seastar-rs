use std::boxed::Box;
use std::cell::{UnsafeCell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::thread;
use std::task::Context;
use std::task::Poll;

use super::waker::noop_waker;
use super::waker::SeastarWaker;

type BoxedFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;
type BoxedTask = Box<Task>;

pub struct Task {
    pub future: BoxedFuture,
}

pub fn spawn(f: impl Future<Output = ()> + 'static) {
    let task = Task {
        future: Box::pin(f),
    };
    let waker = SeastarWaker::from_task(task);
    super::super::spawn(move || {
        waker.wake_by_ref();
    });
}
