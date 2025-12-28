pub mod component;
pub use component::*;

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

use gtk::glib::clone;
use std::sync::LazyLock;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

pub fn state<T>(initial: T) -> State<T> {
    State::new(initial)
}

pub fn effect<E>(e: E)
where
    E: Fn() + 'static,
{
    let effect = Effect::new(e);
    Effect::set_active(Some(effect.clone()));
    effect.call();
    Effect::set_active(None);
}

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

pub fn background<T, F, Fut>(f: F) -> State<T>
where
    T: Clone + Default + 'static,
    F: FnOnce(mpsc::Sender<T>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    let state = state(T::default());
    let (sender, receiver) = mpsc::channel(64);

    RT.spawn(f(sender));

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
