use crate::Effect;

#[derive(Default)]
pub(crate) struct StateInner<T> {
    pub(super) value: T,
    pub(crate) effects: Vec<Effect>,
}

impl<T> StateInner<T> {
    pub(super) fn new(value: T) -> Self {
        let effects = Default::default();

        Self { value, effects }
    }

    pub(super) fn run_effects(&mut self) {
        self.effects.iter_mut().for_each(|e| e.call());
    }

    pub(super) fn add_active_effect(&mut self) {
        if let Some(effect) = Effect::active() {
            self.effects.push(effect);
        }
    }
}
