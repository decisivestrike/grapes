use gtk::{
    Widget,
    gdk::{self, prelude::DisplayExt},
    glib::object::Cast,
    prelude::{BoxExt, WidgetExt},
};
use std::iter::successors;

pub trait GrapesBoxExt {
    fn append_ref(&self, child: impl AsRef<Widget>);

    fn children(&self) -> impl Iterator<Item = Widget>;
}

impl GrapesBoxExt for gtk::Box {
    fn append_ref(&self, child: impl AsRef<Widget>) {
        self.append(child.as_ref());
    }

    fn children(&self) -> impl Iterator<Item = Widget> {
        successors(self.first_child(), |child| child.next_sibling())
    }
}

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
