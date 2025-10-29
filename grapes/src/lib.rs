pub mod component;
pub mod effect;
pub mod extensions;
pub mod reactivity;
pub mod service;
pub mod state;

pub use component::*;
pub use effect::*;
pub use service::*;
pub use state::*;

pub use grapes_macros::*;

pub use gtk;
pub use gtk::glib;

pub use tokio;

use std::sync::LazyLock;
use tokio::runtime::Runtime;

pub static RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());
