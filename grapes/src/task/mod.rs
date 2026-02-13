mod subscribable;
pub use subscribable::SubscribableTask;

use crate::RT;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

// impl Trait in type aliases is still unstable :(
// type TaskFuture = impl Future<Output = ()> + Send + 'static;
pub trait TaskFuture: Future<Output = ()> + Send + 'static {}
impl<T> TaskFuture for T where T: Future<Output = ()> + Send + 'static {}

/// Async background task
#[derive(Debug)]
pub struct Task {
    token: CancellationToken,
}

impl Task {
    pub fn new<F>(f: impl FnOnce(CancellationToken) -> F) -> Self
    where
        F: TaskFuture,
    {
        let token = CancellationToken::new();
        RT.spawn(f(token.clone()));

        Task { token }
    }

    pub fn subscribable<T, F>(
        f: impl FnOnce(broadcast::Sender<T>, CancellationToken) -> F,
    ) -> SubscribableTask<T>
    where
        T: Clone,
        F: TaskFuture,
    {
        SubscribableTask::new(f)
    }

    pub(crate) fn from_parts(token: CancellationToken) -> Self {
        Task { token }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        self.token.cancel();
    }
}
