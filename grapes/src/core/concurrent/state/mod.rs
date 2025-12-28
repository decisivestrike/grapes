pub(crate) mod inner;

use crate::{RT, core::State};
use gtk::glib::{
    self,
    clone::{Downgrade, Upgrade},
};
use std::{fmt, sync::Arc};
use tokio::{
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, mpsc},
    task::JoinHandle,
};

/// Reactive state with counter clone semantic
#[derive(Default, glib::Downgrade)]
pub struct ConcurrentState<T>(Arc<RwLock<StateInner<T>>>);

impl<T> ConcurrentState<T> {
    pub fn new(value: T) -> Self {
        let inner = Arc::new(StateInner::new(value).into());

        Self(inner)
    }

    pub fn spawn_listener_local(
        &self,
        mut receiver: mpsc::Receiver<T>,
    ) -> JoinHandle<()>
    where
        T: Send + Sync + 'static,
    {
        let weak_state = self.downgrade();

        RT.spawn(async move {
            while let Some(value) = receiver.recv().await
                && let Some(state) = &weak_state.upgrade()
            {
                state.set(value)
            }
        })
    }
}

impl<T> State<T> for ConcurrentState<T> {
    type Value<'a>
        = RwLockReadGuard<'a, T>
    where
        T: 'a;

    fn get(&self) -> RwLockReadGuard<'_, T> {
        self.0.blocking_write().add_active_effect();
        self.get_untracked()
    }

    fn get_untracked(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard::map(self.0.blocking_read(), |si| &si.value)
    }

    fn set(&self, value: T) {
        let mut inner = self.0.blocking_write();
        inner.value = value;
    }

    fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        {
            let guard = self.0.blocking_write();
            updater(&mut RwLockWriteGuard::map(guard, |si| &mut si.value));
        }

        self.0.blocking_write().add_active_effect();
        self.0.blocking_read().run_effects();
    }
}

impl<T> Clone for ConcurrentState<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for ConcurrentState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for ConcurrentState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
