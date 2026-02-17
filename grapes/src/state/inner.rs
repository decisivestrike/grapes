use crate::Effect;

#[derive(Default)]
pub(crate) struct StateInner<T> {
    pub(crate) value: T,
    pub(crate) effects: Vec<Effect>,
}

impl<T> StateInner<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            effects: Vec::new(),
        }
    }

    pub(crate) fn run_effects(&mut self) {
        self.effects.iter_mut().for_each(|e| e.call());
    }

    /// Adds active effect to vec
    pub(crate) fn add_active_effect(&mut self) {
        if let Some(effect) = Effect::active() {
            self.effects.push(effect);
        }
    }
}
