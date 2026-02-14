use crate::{RT, State};
use gtk::glib::{self, clone::Downgrade};
use std::{
    rc::Rc,
    sync::{Arc, mpsc::SendError},
};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

pub struct LazyTask<T, TaskFn, F>
where
    T: Clone + Send + 'static,
    TaskFn: Fn(broadcast::Sender<T>) -> F + Send + Sync + 'static,
    F: Future<Output = Result<(), SendError<T>>> + Send + 'static,
{
    f: Arc<TaskFn>,
    sender: broadcast::Sender<T>,
    maybe_handle: Option<JoinHandle<()>>,
}

impl<T, TaskFn, F> LazyTask<T, TaskFn, F>
where
    T: Clone + Send + 'static,
    TaskFn: Fn(broadcast::Sender<T>) -> F + Send + Sync + 'static,
    F: Future<Output = Result<(), SendError<T>>> + Send + 'static,
{
    pub fn new(f: TaskFn) -> Self {
        Self {
            f: Arc::new(f),
            sender: broadcast::Sender::new(64),
            maybe_handle: None,
        }
    }

    pub fn bind(&mut self, state: &Rc<State<T>>) {
        match &self.maybe_handle {
            Some(_) => {
                let weak_state = state.downgrade();
                let mut receiver = self.sender.subscribe();

                glib::spawn_future_local(async move {
                    while let Ok(value) = receiver.recv().await
                        && let Some(state) = &weak_state.upgrade()
                    {
                        state.set(value)
                    }
                });
            }
            None => {
                let sender = self.sender.clone();
                let f = self.f.clone();

                let handle = RT.spawn(async move {
                    loop {
                        if let Err(_) = f(sender.clone()).await {
                            break;
                        }
                    }
                });

                self.maybe_handle = Some(handle);
            }
        }
    }
}
