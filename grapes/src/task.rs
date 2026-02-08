use std::sync::Arc;

use crate::RT;

use tokio::{sync::broadcast, task::JoinHandle};

// impl Trait in type aliases is still unstable :(
// type TaskFuture = impl Future<Output = ()> + Send + 'static;

pub trait TaskFuture: Future<Output = ()> + Send + 'static {}
impl<T> TaskFuture for T where T: Future<Output = ()> + Send + 'static {}

pub trait TaskFn<T, F>: FnOnce(broadcast::Sender<T>) -> F
where
    F: TaskFuture,
{
}
impl<A, T, F> TaskFn<T, F> for A
where
    A: FnOnce(broadcast::Sender<T>) -> F,
    F: TaskFuture,
{
}

pub trait ChainFn<T, F>:
    FnOnce(broadcast::Receiver<T>, broadcast::Sender<T>) -> F
where
    F: TaskFuture,
{
}
impl<A, T, F> ChainFn<T, F> for A
where
    A: FnOnce(broadcast::Receiver<T>, broadcast::Sender<T>) -> F,
    F: TaskFuture,
{
}

/// Async background task
#[derive(Debug, Clone)]
pub struct Task<T>
where
    T: Clone + 'static,
{
    sender: broadcast::Sender<T>,
    handle: Arc<JoinHandle<()>>,
}

impl<T> Task<T>
where
    T: Clone + 'static,
{
    pub(crate) fn new<F>(f: impl TaskFn<T, F>) -> Self
    where
        F: TaskFuture,
    {
        let (sender, _) = broadcast::channel(64);
        let handle = RT.spawn(f(sender.clone())).into();

        Task { sender, handle }
    }

    pub(crate) fn chained<F>(
        receiver: broadcast::Receiver<T>,
        f: impl ChainFn<T, F>,
    ) -> Self
    where
        F: TaskFuture,
    {
        let (sender, _) = broadcast::channel(64);
        let handle = RT.spawn(f(receiver, sender.clone())).into();

        Task { sender, handle }
    }

    pub fn handle(&self) -> Arc<JoinHandle<()>> {
        self.handle.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.sender.subscribe()
    }
}
