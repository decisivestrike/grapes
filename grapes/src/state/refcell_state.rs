use crate::{effect::Effect, state::InnerState};
use std::cell::RefCell;

struct RefCellState<T> {
    value: RefCell<T>,
    effects: RefCell<Vec<Effect>>,
}

impl<T: 'static> InnerState<T> for RefCellState<T> {
    fn new(value: T) -> Self {
        let value = RefCell::new(value);
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
