use gtk::{
    gdk::{self, prelude::DisplayExt},
    glib::object::Cast,
};

pub trait GrapesMonitorExt {
    fn all() -> Vec<gdk::Monitor>;
}

impl GrapesMonitorExt for gdk::Monitor {
    fn all() -> Vec<gdk::Monitor> {
        let display = gdk::Display::default().expect("No display");

        display
            .monitors()
            .into_iter()
            .filter_map(|obj| obj.ok()?.downcast::<gdk::Monitor>().ok())
            .collect()
    }
}
