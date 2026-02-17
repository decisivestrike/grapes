pub(crate) mod inner;

use crate::{Effect, inner::StateInner};
use core::fmt;
use gtk::glib::{self, Downgrade, clone::Downgrade};
use std::{
    cell::UnsafeCell,
    fmt::{Debug, Display},
    rc::Rc,
};
use tokio::sync::broadcast;

/// Reactive state with counter clone semantic
#[derive(Clone, Default, Downgrade)]
pub struct State<T>(Rc<UnsafeCell<StateInner<T>>>);

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        let inner: UnsafeCell<_> = StateInner::new(value).into();

        Self(Rc::new(inner))
    }

    /// Getter
    pub fn get(&self) -> &T {
        self.inner_mut().add_active_effect();

        if let Some(effect) = Effect::active() {
            effect.register(self);
        }

        self.get_untracked()
    }

    /// Get without adding active effect
    pub fn get_untracked(&self) -> &T {
        &self.inner().value
    }

    /// Setter
    pub fn set(&self, value: T) {
        self.inner_mut().value = value;
        self.inner_mut().run_effects();
    }

    pub fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        updater(&mut self.inner_mut().value);

        self.inner_mut().add_active_effect();
        self.inner_mut().run_effects();
    }

    /// Spawn local future which listen receiver and update state when receiving messages
    pub fn track(
        self: &Rc<Self>,
        sender: &broadcast::Sender<T>,
    ) -> glib::JoinHandle<()>
    where
        T: Clone + 'static,
    {
        let weak_state = self.downgrade();
        let mut receiver = sender.subscribe();

        glib::spawn_future_local(async move {
            while let Ok(value) = receiver.recv().await
                && let Some(state) = &weak_state.upgrade()
            {
                state.set(value)
            }
        })
    }

    pub(crate) fn inner(&self) -> &StateInner<T> {
        unsafe { &*self.0.get() }
    }

    pub(crate) fn inner_mut(&self) -> &mut StateInner<T> {
        unsafe { &mut *self.0.get() }
    }
}

impl<T: Debug> fmt::Debug for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: Display> fmt::Display for State<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
