pub trait Component: AsRef<gtk::Widget> + Clone + 'static {
    fn as_widget_ref(&self) -> &gtk::Widget;
}

pub trait WindowComponent {
    fn present(&self);

    fn destroy(&self);
}

pub trait UpdateableComponent: 'static {
    type Message: Clone + 'static;

    fn update(&self, message: Self::Message);
}
