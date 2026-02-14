use tokio::task::JoinHandle;

mod respawnable;
pub use respawnable::RespawnableTask;

use crate::RT;
use tokio_util::sync::CancellationToken;

/// Simple RAII wraper for tokio task
#[derive(Debug)]
pub struct Task<T> {
    handle: JoinHandle<T>,
    token: CancellationToken,
}

impl<T> Task<T> {
    /// You can pass here the CancellationToken that you use inside the closure
    pub fn new<F>(f: impl FnOnce(CancellationToken) -> F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let token = CancellationToken::new();
        let handle = RT.spawn(f(token.clone()));

        Task { token, handle }
    }

    pub fn handle(&self) -> &JoinHandle<T> {
        &self.handle
    }
}

impl<T> Drop for Task<T> {
    fn drop(&mut self) {
        self.token.cancel();
    }
}
