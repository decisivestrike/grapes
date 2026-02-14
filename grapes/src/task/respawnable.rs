use tokio_util::sync::CancellationToken;

use crate::Task;

pub struct RespawnableTask<T, F, Fut>
where
    F: Fn(CancellationToken) -> Fut,
    T: Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    f: F,
    maybe_task: Option<Task<T>>,
}

impl<T, F, Fut> RespawnableTask<T, F, Fut>
where
    F: Fn(CancellationToken) -> Fut,
    T: Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
{
    pub fn new(f: F) -> Self {
        let task = Task::new(&f);

        Self {
            f,
            maybe_task: Some(task),
        }
    }

    pub fn respawn(&mut self) {
        if self.maybe_task.is_none() {
            let task = Task::new(&self.f);
            self.maybe_task = Some(task);
        }
    }

    pub fn kill(&mut self) {
        _ = self.maybe_task.take();
    }
}
