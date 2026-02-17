use crate::Effect;
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct StateInner<T> {
    pub(crate) value: T,
    pub(crate) effects: HashSet<Effect>,
}

impl<T> StateInner<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            effects: Default::default(),
        }
    }

    pub(crate) fn run_effects(&mut self) {
        for effect in self.effects.iter() {
            effect.call();
        }
    }

    /// Adds active effect to set
    ///
    /// Returns whether the value was newly inserted
    pub(crate) fn add_active_effect(&mut self) -> bool {
        if let Some(effect) = Effect::active() {
            self.effects.insert(effect)
        } else {
            false
        }
    }
}
