use gtk::CssProvider;
use std::path::Path;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StylePriority {
    Fallback = 1,
    Theme = 200,
    Settings = 400,
    Application = 600,
    User = 800,
}

#[derive(Debug, Clone)]
pub struct Css {
    provider: CssProvider,
}

impl Css {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(path);

        Self { provider }
    }

    pub fn apply(&self, priority: StylePriority) {
        let display = gtk::gdk::Display::default()
            .expect("Could not connect to a display.");

        gtk::style_context_add_provider_for_display(
            &display,
            &self.provider,
            priority as u32,
        );
    }
}
