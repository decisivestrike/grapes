mod inner;

use crate::{State, effect::inner::EffectInner};
use std::{cell::RefCell, rc::Rc};

thread_local! {
    static EFFECT_COUNT: RefCell<u32> = RefCell::new(0);
    static ACTIVE_EFFECT: RefCell<Option<Effect>> = RefCell::new(None);
}

/// Effects are functions that run when state updates
#[derive(Clone)]
pub struct Effect {
    id: u32,
    inner: Rc<RefCell<EffectInner>>,
}

impl Effect {
    pub(crate) fn new<F>(f: F) -> Self
    where
        F: Fn() + 'static,
    {
        let effect = Self {
            id: Self::count(),
            inner: Rc::new(RefCell::new(EffectInner::new(f))),
        };

        Self::increment();

        effect
    }

    /// Get global active effect
    pub(crate) fn active() -> Option<Effect> {
        ACTIVE_EFFECT.with_borrow(|maybe_effect| match maybe_effect {
            Some(effect) => Some(effect.clone()),
            None => None,
        })
    }

    pub(crate) fn set_active(maybe_effect: Option<Effect>) {
        ACTIVE_EFFECT.with_borrow_mut(|e| *e = maybe_effect);
    }

    pub(crate) fn register<T>(&self, state: &State<T>) {}

    pub(crate) fn call(&self) {
        self.inner.borrow().call();
    }

    pub(crate) fn count() -> u32 {
        EFFECT_COUNT.with_borrow(|c| *c)
    }

    pub(crate) fn increment() {
        EFFECT_COUNT.with_borrow_mut(|count| *count += 1);
    }

    pub(crate) fn decrement() {
        EFFECT_COUNT.with_borrow_mut(|count| *count -= 1);
    }
}

impl Drop for Effect {
    fn drop(&mut self) {
        Self::decrement();
    }
}

impl PartialEq for Effect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Effect {}
