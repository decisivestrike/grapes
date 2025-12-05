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

/// Run tests in single thread
///
/// `cargo test -- --test-threads=1`
#[cfg(test)]
mod tests {

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
