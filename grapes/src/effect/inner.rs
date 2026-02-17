use crate::State;
use std::rc::Weak;

trait ContainsEffect {
    fn remove_effect(&self, id: u32);
}

impl<T> ContainsEffect for State<T> {
    fn remove_effect(&self, id: u32) {
        self.inner_mut().effects.retain(|e| e.id != id);
    }
}

pub struct EffectInner {
    f: Box<dyn Fn() + 'static>,
    deps: Vec<Weak<dyn ContainsEffect>>,
}

impl EffectInner {
    pub fn new(f: impl Fn() + 'static) -> Self {
        Self {
            f: Box::new(f),
            deps: Vec::new(),
        }
    }

    pub fn call(&self) {
        (self.f)();
    }
}
