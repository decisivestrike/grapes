pub trait GtkCompatible: AsRef<gtk::Widget> + Clone + 'static {
    fn as_widget_ref(&self) -> &gtk::Widget;
}

pub trait Component: GtkCompatible {
    const NAME: &str;

    fn name(&self) -> &str {
        Self::NAME
    }
}

pub trait WindowComponent {
    fn present(&self);

    fn destroy(&self);
}
