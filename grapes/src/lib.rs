pub mod component;
pub use component::*;

pub mod css;
pub use css::Css;

pub mod effect;
pub use effect::*;

pub mod extensions;

pub mod reactive;
pub use reactive::Reactive;

pub mod service;
pub use service::*;

pub mod state;
pub use state::*;

pub mod connectable;
pub use connectable::Connectable;

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

use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

/// The tests should be run in a single thread.
///
/// `cargo test -- --test-threads=1`
#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::glib::clone;
    use crate::prelude::*;
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
    fn state_and_effect() {
        let state = state(0);

        let sum: Rc<Cell<i32>> = Default::default();

        effect(clone!(
            #[strong]
            sum,
            move || sum += *state.get()
        ));
    }

    #[test]
    fn state_get_twice() {
        let state = state(0);

        effect(clone!(
            #[strong]
            state,
            move || {
                state.get();
                // Don't use `get()` twice (it will also be added)
                state.get();
            }
        ));

        assert_eq!(state.effects().len(), 2);
    }

    #[test]
    fn ext_monitor_all() {
        gtk_safe_init();

        let monitors = Monitor::all();
        assert!(!monitors.is_empty());
    }

    #[test]
    fn ext_box_children() {
        gtk_safe_init();

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let label_1 = gtk::Label::new(None);
        container.append(&label_1);

        let label_2 = gtk::Label::new(None);
        container.append(&label_2);

        let mut children = container.children();

        assert_eq!(children.next(), Some(label_1.upcast::<gtk::Widget>()));
        assert_eq!(children.next(), Some(label_2.upcast::<gtk::Widget>()));
        assert_eq!(children.next(), None);
    }
}
