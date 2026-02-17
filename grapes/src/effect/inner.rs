use crate::State;
use gtk::glib::clone::Downgrade;
use std::hash::{Hash, Hasher};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

thread_local! {
    static EFFECT_COUNT: RefCell<u32> = RefCell::new(0);
}

trait ContainsEffect {
    fn remove_effect(&self, id: u32);
}

impl<T> ContainsEffect for State<T> {
    fn remove_effect(&self, id: u32) {
        self.inner_mut().effects.retain(|e| e.id() != id);
    }
}

pub struct EffectInner {
    pub(super) id: u32,
    pub(super) f: Box<dyn Fn() + 'static>,
    weak_deps: RefCell<Vec<Weak<dyn ContainsEffect>>>,
}

/// Interior mutability
impl EffectInner {
    pub fn new(f: impl Fn() + 'static) -> Self {
        let effect = Self {
            id: Self::count(),
            f: Box::new(f),
            weak_deps: Default::default(),
        };

        Self::increment();

        effect
    }

    /// Adds state as dependency
    pub fn add_dep<T>(&self, state: &Rc<State<T>>) {
        let weak_state = state.downgrade();
        self.weak_deps.borrow_mut().push(weak_state);
    }

    pub fn clear_deps(&self) {
        let mut weak_deps = self.weak_deps.borrow_mut();

        for weak_dep in weak_deps.iter_mut() {
            if let Some(dep) = weak_dep.upgrade() {
                dep.remove_effect(self.id);
            }
        }

        weak_deps.clear();
    }

    /// Calls effect fn
    pub fn call(&self) {
        (self.f)();
    }
}

impl EffectInner {
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

impl Drop for EffectInner {
    fn drop(&mut self) {
        Self::decrement();
    }
}

impl PartialEq for EffectInner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for EffectInner {}

impl Hash for EffectInner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
