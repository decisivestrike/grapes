use gtk::{
    CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
    STYLE_PROVIDER_PRIORITY_FALLBACK, STYLE_PROVIDER_PRIORITY_SETTINGS,
    STYLE_PROVIDER_PRIORITY_THEME, STYLE_PROVIDER_PRIORITY_USER,
};
use std::path::Path;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StylePriority {
    Fallback = STYLE_PROVIDER_PRIORITY_FALLBACK,
    Theme = STYLE_PROVIDER_PRIORITY_THEME,
    Settings = STYLE_PROVIDER_PRIORITY_SETTINGS,
    Application = STYLE_PROVIDER_PRIORITY_APPLICATION,
    User = STYLE_PROVIDER_PRIORITY_USER,
    Custom(u32),
}

impl From<StylePriority> for u32 {
    fn from(priority: StylePriority) -> u32 {
        match priority {
            StylePriority::Fallback => STYLE_PROVIDER_PRIORITY_FALLBACK,
            StylePriority::Theme => STYLE_PROVIDER_PRIORITY_THEME,
            StylePriority::Settings => STYLE_PROVIDER_PRIORITY_SETTINGS,
            StylePriority::Application => STYLE_PROVIDER_PRIORITY_APPLICATION,
            StylePriority::User => STYLE_PROVIDER_PRIORITY_USER,
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
        let display = gtk::gdk::Display::default()
            .expect("Could not connect to a display.");

        gtk::style_context_add_provider_for_display(
            &display,
            &self.provider,
            priority.into(),
        );
    }
}
