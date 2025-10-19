use core::fmt;
use gtk::glib::clone;
use std::{
    cell::{Ref, RefCell},
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
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

struct StateInner<T> {
    value: RefCell<T>,
    effects: RefCell<Vec<Effect>>,
}

impl<T> StateInner<T> {
    fn new(value: T) -> Self {
        let value = RefCell::new(value);
        let effects = RefCell::new(vec![]);

        Self { value, effects }
    }

    fn get_untracked(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    fn get(&self) -> Ref<'_, T> {
        self.handle_active_effect();
        self.get_untracked()
    }

    fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        self.run_effects();
    }

    fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        updater(&mut self.value.borrow_mut());

        self.handle_active_effect();
        self.run_effects();
    }

    fn run_effects(&self) {
        for effect in self.effects.borrow().iter() {
            effect.call();
        }
    }

    fn handle_active_effect(&self) {
        if let Some(effect) = Effect::active() {
            self.effects.borrow_mut().push(effect);
        }
    }
}

/// Reactive state with counter clone semantic
pub struct State<T> {
    inner: Rc<StateInner<T>>,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        let inner = Rc::new(StateInner::new(value));

        Self { inner }
    }

    pub fn get(&self) -> Ref<'_, T> {
        self.inner.get()
    }

    pub fn get_untracked(&self) -> Ref<'_, T> {
        self.inner.get_untracked()
    }

    pub fn set(&self, value: T) {
        self.inner.set(value);
    }

    pub fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        self.inner.update(updater);
    }
}

/// Absolutely stupid clone cuz `#[derive(Clone)]` doesnt work
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

/// Ops
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
