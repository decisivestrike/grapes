use gtk::glib::{
    self,
    clone::{Downgrade, Upgrade},
};
use tokio::sync::mpsc;

use crate::State;

pub(crate) struct LocalFuture;

impl LocalFuture {
    pub(crate) fn spawn_state_listener<T>(
        state: &State<T>,
        mut receiver: mpsc::Receiver<T>,
    ) where
        T: 'static,
    {
        let weak_state = state.downgrade();

        glib::spawn_future_local(async move {
            while let Some(value) = receiver.recv().await
                && let Some(state) = &weak_state.upgrade()
            {
                state.set(value)
            }
        });
    }
}
