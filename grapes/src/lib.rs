//! Reactivity for gtk4-rs
//!
//! Provides reactivity primitives and macros for component approach

pub mod component;
pub use component::*;

mod task;
pub use task::Task;

pub mod css;
pub use css::Css;

pub mod effect;
pub use effect::*;

pub mod extensions;

pub mod reactive;
pub use reactive::Reactive;

pub mod state;
pub use state::*;

pub mod prelude;

pub use grapes_macros::*;

pub use gtk;
pub use gtk::cairo;
pub use gtk::gio;
pub use gtk::glib;
pub use gtk::pango;
pub use layer_shell;
pub use tokio;

use gtk::glib::clone::Downgrade;
use std::rc::Rc;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

/// Creates reactive state wrapped by Rc
///
/// ```rust
/// use grapes::state;
///
/// let count = state(0);
/// count.set(1);
///
/// assert_eq!(*count.get(), 1);
/// ```
///
/// Use it with effects
pub fn state<T>(initial: T) -> Rc<State<T>> {
    State::new(initial).into()
}

/// Creates effect
pub fn effect<E>(e: E)
where
    E: Fn() + 'static,
{
    let effect = Effect::new(e);
    Effect::set_active(Some(effect.clone()));
    effect.call();
    Effect::set_active(None);
}

/// Creates state which depends on another state
///
/// Uses effect with weak ref
pub fn derived<T, F>(f: F) -> Rc<State<T>>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    let state = state(f());

    effect({
        let weak_state = state.downgrade();
        move || {
            if let Some(state) = weak_state.upgrade() {
                state.set(f())
            }
        }
    });

    state
}

/// Run tests in a single thread!
///
/// `cargo test -- --test-threads=1`
#[cfg(test)]
mod tests {
    use crate::*;
    use gtk::glib::clone;
    use std::cell::Cell;

    static mut IS_INIT: bool = false;

    fn gtk_safe_init() {
        unsafe {
            if !IS_INIT {
                gtk::init().unwrap();
                IS_INIT = true;
            }
        }
    }

    #[test]
    fn state_get() {
        gtk_safe_init();

        let state = state(0);

        assert_eq!(*state.get(), 0);
    }

    #[test]
    fn state_set() {
        gtk_safe_init();

        let state = state(0);
        state.set(137);

        assert_eq!(*state.get(), 137);
    }

    #[test]
    fn state_set_with_effect() {
        gtk_safe_init();

        let state = state(0);
        let count = Rc::new(Cell::new(0));

        effect(clone!(
            #[weak]
            state,
            #[weak]
            count,
            move || count.set(*state.get())
        ));

        state.set(52);

        assert_eq!(count.get(), 52);
    }

    #[test]
    fn double_get_in_effect() {
        gtk_safe_init();

        let state1 = state(1);
        let state2 = state(1);

        effect(clone!(
            #[weak]
            state1,
            #[weak]
            state2,
            move || {
                _ = state1.get();
                state2.update(|n| *n += state1.get());
            }
        ));

        state1.set(2);

        assert_eq!(state1.effect_count(), 1);
        assert_eq!(state2.effect_count(), 0);

        assert_eq!(*state2.get(), 4); // 1 + 1 + 2
    }

    #[test]
    fn update() {
        gtk_safe_init();

        let state1 = state(0);
        let state2 = state(1);

        effect(clone!(
            #[weak]
            state1,
            #[weak]
            state2,
            move || {
                state1.update(|n| *n += state2.get());
            }
        ));

        state2.set(1);
        state2.set(1);

        assert_eq!(*state1.get(), 3);
    }

    #[test]
    fn effect_deactivate() {
        gtk_safe_init();

        let state1 = state(1);
        let state2 = state(2);
        let count = Rc::new(Cell::new(0));

        effect(clone!(
            #[weak]
            state1,
            #[weak]
            state2,
            #[weak]
            count,
            move || {
                let sum = state1.get() + state2.get();
                count.set(sum);
            }
        ));

        state1.set(2);
        assert_eq!(count.get(), 4);

        drop(state1);

        state2.set(42);

        assert_eq!(count.get(), 4);
        assert_eq!(state2.effect_count(), 0);
    }

    #[test]
    fn simple_derived() {
        gtk_safe_init();

        let count = state(0);
        let double = derived(clone!(
            #[strong]
            count,
            move || count.get() * 2
        ));

        count.set(2);

        assert_eq!(*double.get(), 4);

        assert_eq!(count.effect_count(), 1);
        assert_eq!(double.effect_count(), 0);
    }

    #[test]
    fn strong_derived() {
        gtk_safe_init();

        let first = state(0);
        let second = state(0);
        let sum = derived(clone!(
            #[strong]
            first,
            #[strong]
            second,
            move || first.get() + second.get()
        ));

        first.set(2);
        assert_eq!(*sum.get(), 2);

        second.set(2);
        assert_eq!(*sum.get(), 4);

        assert_eq!(first.effect_count(), 1);
        assert_eq!(second.effect_count(), 1);
        assert_eq!(sum.effect_count(), 0);

        drop(first);

        second.set(10);
        assert_eq!(*sum.get(), 12);

        assert_eq!(second.effect_count(), 1); // Because strong ref
    }
}
