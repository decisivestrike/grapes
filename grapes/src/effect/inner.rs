use crate::State;
use gtk::glib::clone::Downgrade;
use std::rc::{Rc, Weak};

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
            deps: Default::default(),
        }
    }

    /// Adds state as dependency
    pub fn add_state<T>(&mut self, state: &Rc<State<T>>) {
        let weak_state = state.downgrade();
        self.deps.push(weak_state);
    }

    /// Calls effect fn
    pub fn call(&self) {
        (self.f)();
    }
}
