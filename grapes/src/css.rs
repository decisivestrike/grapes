use gtk::CssProvider;
use std::path::Path;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StylePriority {
    Fallback = gtk::STYLE_PROVIDER_PRIORITY_FALLBACK,
    Theme = gtk::STYLE_PROVIDER_PRIORITY_THEME,
    Settings = gtk::STYLE_PROVIDER_PRIORITY_SETTINGS,
    Application = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    User = gtk::STYLE_PROVIDER_PRIORITY_USER,
    Custom(u32),
}

impl From<StylePriority> for u32 {
    fn from(priority: StylePriority) -> u32 {
        match priority {
            StylePriority::Fallback => gtk::STYLE_PROVIDER_PRIORITY_FALLBACK,
            StylePriority::Theme => gtk::STYLE_PROVIDER_PRIORITY_THEME,
            StylePriority::Settings => gtk::STYLE_PROVIDER_PRIORITY_SETTINGS,
            StylePriority::Application => gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            StylePriority::User => gtk::STYLE_PROVIDER_PRIORITY_USER,
            StylePriority::Custom(v) => v,
        }
    }
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

    pub fn from_str(raw: &str) -> Self {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(raw);

        Self { provider }
    }

    pub fn apply(&self, priority: StylePriority) {
        let display =
            gtk::gdk::Display::default().expect("Could not connect to a display.");

        gtk::style_context_add_provider_for_display(
            &display,
            &self.provider,
            priority.into(),
        );
    }
}
