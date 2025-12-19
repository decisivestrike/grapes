use core::fmt;
use gtk::glib::{self, clone};
use std::{
    cell::{RefCell, UnsafeCell},
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::effect::{Effect, effect};

pub fn state<T>(initial: T) -> State<T> {
    State::new(initial)
}

pub fn derived<T, F>(f: F) -> State<T>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    let state = State::new(f());

    effect(clone!(
        #[strong]
        state,
        move || state.set(f())
    ));

    state
}

// background

#[derive(Default)]
struct StateInner<T> {
    value: UnsafeCell<T>,
    effects: RefCell<Vec<Effect>>,
}

impl<T> StateInner<T> {
    fn new(value: T) -> Self {
        let value = UnsafeCell::new(value);
        let effects = Default::default();

        Self { value, effects }
    }

    fn run_effects(&self) {
        for effect in self.effects.borrow().iter() {
            effect.call();
        }
    }

    fn add_active_effect(&self) {
        if let Some(effect) = Effect::active() {
            self.effects.borrow_mut().push(effect);
        }
    }
}

/// Reactive state with counter clone semantic
#[derive(Default, glib::Downgrade)]
pub struct State<T> {
    inner: Rc<StateInner<T>>,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        let inner = Rc::new(StateInner::new(value));

        Self { inner }
    }

    pub fn get(&self) -> &T {
        self.inner.add_active_effect();
        self.get_untracked()
    }

    pub fn get_untracked(&self) -> &T {
        unsafe { &*self.inner.value.get() }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.inner.value.get() = value;
        }
        self.inner.run_effects();
    }

    pub fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        unsafe {
            updater(&mut *self.inner.value.get());
        }

        self.inner.add_active_effect();
        self.inner.run_effects();
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug> fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: Display> fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
