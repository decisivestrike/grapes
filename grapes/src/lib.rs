pub mod component;
pub use component::*;

pub mod task;

pub mod css;
pub use css::Css;

pub mod effect;
pub use effect::*;

pub mod extensions;

pub mod reactive;
pub use reactive::Reactive;

pub mod state;
pub use state::*;

pub mod timing;

pub mod updateable;
pub use updateable::Updateable;

pub mod prelude;

pub use grapes_macros::*;

pub use gtk;
pub use gtk::cairo;
pub use gtk::gio;
pub use gtk::glib;
pub use gtk::pango;
pub use layer_shell;
pub use tokio;

use crate::task::ChainFn;
use crate::task::Task;
use crate::task::TaskFn;
use crate::task::TaskFuture;
use gtk::glib::clone;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

/// Creates state
#[inline]
pub fn state<T>(initial: T) -> State<T> {
    State::new(initial)
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
pub fn derived<T, F>(f: F) -> State<T>
where
    F: Fn() -> T + 'static,
    T: 'static,
{
    let state = State::new(f());

    effect(clone!(
        #[strong]
        state,
        move || state.set(f())
    ));

    state
}

/// Creates async background task
#[inline]
pub fn task<T, F>(f: impl TaskFn<T, F>) -> Task<T>
where
    T: Clone + 'static,
    F: TaskFuture,
{
    Task::new(f)
}

/// Creates task which listen another task channel
#[inline]
pub fn chain<T, F>(parent: &Task<T>, f: impl ChainFn<T, F>) -> Task<T>
where
    T: Clone + 'static,
    F: TaskFuture,
{
    let receiver = parent.subscribe();

    Task::chained(receiver, f)
}

/// Creates state which listen task channel
pub fn subscriber<T>(task: &Task<T>) -> State<T>
where
    T: Clone + Default,
{
    let state = state(T::default());
    let receiver = task.subscribe();

    state.spawn_listener_local(receiver);

    state
}

/// Run tests in single thread
///
/// `cargo test -- --test-threads=1`
#[cfg(test)]
mod tests {
    use crate::prelude::{monitor::GrapesMonitorExt, *};
    use gtk::gdk::Monitor;

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
    fn test_monitors_all() {
        gtk_safe_init();

        let monitors = Monitor::all();
        assert!(!monitors.is_empty());
    }

    #[test]
    fn test_monitors_all2() {
        gtk_safe_init();

        let monitors = Monitor::all();
        assert!(!monitors.is_empty());
    }
}
