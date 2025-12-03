use gtk::{Application, gdk::Monitor};

pub trait GtkCompatible: AsRef<gtk::Widget> + Clone + 'static {
    fn as_widget_ref(&self) -> &gtk::Widget;
}

pub trait Component: GtkCompatible {
    const NAME: &str;

    type Props;

    fn new(props: Self::Props) -> Self;

    fn name(&self) -> &str {
        Self::NAME
    }
}

pub trait WindowComponent {
    type Props;

    fn new(app: &Application, monitor: &Monitor, props: Self::Props) -> Self;

    fn present(&self);
}
