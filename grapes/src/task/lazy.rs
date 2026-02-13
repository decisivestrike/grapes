use crate::{RT, State, SubscribableTask, Task, task::TaskFuture};
use std::rc::Rc;
use tokio::sync::{RwLock, broadcast};
use tokio_util::sync::CancellationToken;

pub struct LazyTask<T, C, F>
where
    T: Clone + 'static,
    C: Fn(broadcast::Sender<T>, CancellationToken) -> F,
    F: TaskFuture,
{
    f: C,
    task: RwLock<Option<SubscribableTask<T>>>,
}

impl<T, C, F> LazyTask<T, C, F>
where
    T: Clone + 'static,
    C: Fn(broadcast::Sender<T>, CancellationToken) -> F,
    F: TaskFuture,
{
    pub const fn new(f: C) -> Self {
        Self {
            f,
            task: RwLock::const_new(None),
        }
    }

    pub fn bind(&self, state: &Rc<State<T>>) {
        let mut maybe_task = self.task.blocking_write();

        match &*maybe_task {
            Some(task) => task.bind(state),
            None => {
                let token = CancellationToken::new();
                let (sender, _) = broadcast::channel(64);

                RT.spawn((self.f)(sender.clone(), token.clone()));

                let task = Task::from_parts(token);
                let subscribable_task =
                    SubscribableTask::from_parts(sender, task);

                *maybe_task = Some(subscribable_task);
            }
        }
    }

    pub fn turn_off(&mut self) {
        _ = self.task.blocking_write().take();
    }
}

unsafe impl<T, C, F> Sync for LazyTask<T, C, F>
where
    T: Clone + 'static,
    C: Fn(broadcast::Sender<T>, CancellationToken) -> F,
    F: TaskFuture,
{
}
