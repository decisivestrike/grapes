use core::fmt;
use gtk::glib::clone;
use std::{
    cell::{RefCell, UnsafeCell},
    fmt::{Debug, Display},
    ops::{Add, Deref, Div, Mul, Sub},
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
#[derive(Default)]
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

impl<T> Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

// OPERATORS OVERLOADINGS
impl<T> Add for &State<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = T;

    fn add(self, rhs: Self) -> Self::Output {
        *self.get() + *rhs.get()
    }
}

impl<T> Sub for &State<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = T;

    fn sub(self, rhs: Self) -> Self::Output {
        *self.get() - *rhs.get()
    }
}

impl<T> Mul for &State<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        *self.get() * *rhs.get()
    }
}

impl<T> Div for &State<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = T;

    fn div(self, rhs: Self) -> Self::Output {
        *self.get() / *rhs.get()
    }
}
