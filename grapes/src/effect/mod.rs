mod inner;

use crate::{State, effect::inner::EffectInner};
use std::{cell::RefCell, rc::Rc};

thread_local! {
    static ACTIVE_EFFECT: RefCell<Option<Effect>> = RefCell::new(None);
}

/// Effects are functions that run when state updates
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Effect(Rc<EffectInner>);

impl Effect {
    pub(crate) fn new<F>(f: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self(Rc::new(EffectInner::new(f)))
    }

    pub fn id(&self) -> u32 {
        self.0.id
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

    pub(crate) fn register<T>(&self, state: &Rc<State<T>>)
    where
        T: 'static,
    {
        self.0.add_dep(state);
    }

    /// Вызывается со стороны State, когда зависимости эффекта невалидны
    pub(crate) fn deactivate(&self) {
        self.0.clear_deps();
    }

    pub(crate) fn call(&self) {
        self.0.call();
    }
}
