pub(crate) mod inner;

use core::fmt;
use gtk::glib::{
    self,
    clone::{Downgrade, Upgrade},
};
use inner::StateInner;
use std::{
    cell::UnsafeCell,
    fmt::{Debug, Display},
    rc::Rc,
};
use tokio::sync::mpsc;

use crate::core::State;

/// Reactive state with counter clone semantic
#[derive(Default, glib::Downgrade)]
pub struct LocalState<T>(Rc<UnsafeCell<StateInner<T>>>);

impl<T> LocalState<T> {
    pub fn new(value: T) -> Self {
        let inner = Rc::new(StateInner::new(value).into());

        Self(inner)
    }

    pub fn spawn_listener_local(
        &self,
        mut receiver: mpsc::Receiver<T>,
    ) -> glib::JoinHandle<()>
    where
        T: 'static,
    {
        let weak_state = self.downgrade();

        glib::spawn_future_local(async move {
            while let Some(value) = receiver.recv().await
                && let Some(state) = &weak_state.upgrade()
            {
                state.set(value)
            }
        })
    }

    fn inner(&self) -> &StateInner<T> {
        unsafe { &*self.0.get() }
    }

    fn inner_mut(&self) -> &mut StateInner<T> {
        unsafe { &mut *self.0.get() }
    }
}

impl<T> State<T> for LocalState<T> {
    type Value<'a>
        = &'a T
    where
        T: 'a;

    fn get(&self) -> &T {
        self.inner_mut().add_active_effect();
        self.get_untracked()
    }

    fn get_untracked(&self) -> &T {
        &self.inner().value
    }

    fn set(&self, value: T) {
        self.inner_mut().value = value;
        self.inner().run_effects();
    }

    fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        updater(&mut self.inner_mut().value);

        self.inner_mut().add_active_effect();
        self.inner().run_effects();
    }
}

impl<T> Clone for LocalState<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Debug> fmt::Debug for LocalState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: Display> fmt::Display for LocalState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
