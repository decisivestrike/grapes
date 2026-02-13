use std::rc::Rc;

use crate::{RT, State, Task, state};
use gtk::glib::{self, clone::Downgrade};
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

/// Async background task
#[derive(Debug)]
pub struct SubscribableTask<T>
where
    T: Clone + 'static,
{
    sender: broadcast::Sender<T>,

    /// For drop
    _task: super::Task,
}

impl<T> SubscribableTask<T>
where
    T: Clone + 'static,
{
    pub(crate) fn new<F>(
        f: impl FnOnce(broadcast::Sender<T>, CancellationToken) -> F,
    ) -> Self
    where
        F: super::TaskFuture,
    {
        let (sender, _) = broadcast::channel(64);
        let token = CancellationToken::new();

        RT.spawn(f(sender.clone(), token.clone()));

        SubscribableTask {
            sender,
            _task: Task::from_parts(token),
        }
    }

    /// Creates new state
    pub fn subscriber(&self) -> Rc<State<Option<T>>> {
        let state = state(None);

        let weak_state = state.downgrade();
        let mut receiver = self.sender.subscribe();

        glib::spawn_future_local(async move {
            while let Ok(value) = receiver.recv().await
                && let Some(state) = &weak_state.upgrade()
            {
                state.set(Some(value))
            }
        });

        state
    }

    pub fn sender(&self) -> broadcast::Sender<T> {
        self.sender.clone()
    }
}
