use std::{cell::RefCell, rc::Rc};

thread_local! {
    static ACTIVE_EFFECT: RefCell<Option<Effect>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct Effect(Rc<dyn Fn() + 'static>);

impl Effect {
    pub(crate) fn new<F>(f: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self(Rc::new(f))
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

    pub(crate) fn call(&self) {
        (self.0)();
    }
}
