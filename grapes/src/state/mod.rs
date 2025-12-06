pub mod refcell_state;

#[cfg(test)]
use crate::Effect;
use crate::effect::effect;
use core::fmt;
use gtk::glib::clone;
use std::{
    cell::Ref,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Sub},
    rc::Rc,
};

pub fn state<T: 'static>(initial: T) -> State<T> {
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

trait InnerState<T>
where
    T: 'static,
{
    fn new(value: T) -> Self
    where
        Self: Sized;

    fn run_effects(&self);

    fn add_active_effect(&self);
}

/// Reactive state with counter clone semantic
pub struct State<T> {
    inner: Rc<dyn InnerState<T>>,
}

impl<T: 'static> State<T> {
    pub fn new(value: T) -> Self {
        let inner = Rc::new(RefCellState::new(value));

        Self { inner }
    }

    pub fn get(&self) -> Ref<'_, T> {
        self.inner.add_active_effect();
        self.get_untracked()
    }

    pub fn get_untracked(&self) -> Ref<'_, T> {
        self.inner.value.borrow()
    }

    pub fn set(&self, value: T) {
        *self.inner.value.borrow_mut() = value;
        self.inner.run_effects();
    }

    pub fn update<U>(&self, updater: U)
    where
        U: FnOnce(&mut T),
    {
        updater(&mut self.inner.value.borrow_mut());

        self.inner.add_active_effect();
        self.inner.run_effects();
    }

    #[cfg(test)]
    pub fn effects(&self) -> Ref<'_, Vec<Effect>> {
        self.inner.effects.borrow()
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Debug + 'static> fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T: Display + 'static> fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T> Add for &State<T>
where
    T: Add<Output = T> + Copy + 'static,
{
    type Output = T;

    fn add(self, rhs: Self) -> Self::Output {
        *self.get() + *rhs.get()
    }
}

impl<T> AddAssign for &State<T>
where
    T: Add<Output = T> + Copy + 'static,
{
    fn add_assign(&mut self, rhs: Self) {
        self.update(|s| s + rhs.get());
    }
}

impl<T> Sub for &State<T>
where
    T: Sub<Output = T> + Copy + 'static,
{
    type Output = T;

    fn sub(self, rhs: Self) -> Self::Output {
        *self.get() - *rhs.get()
    }
}

impl<T> Mul for &State<T>
where
    T: Mul<Output = T> + Copy + 'static,
{
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        *self.get() * *rhs.get()
    }
}

impl<T> Div for &State<T>
where
    T: Div<Output = T> + Copy + 'static,
{
    type Output = T;

    fn div(self, rhs: Self) -> Self::Output {
        *self.get() / *rhs.get()
    }
}
