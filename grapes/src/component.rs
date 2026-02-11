use gtk::glib::{WeakRef, object::ObjectExt};

pub trait Component: AsRef<gtk::Widget> + 'static {
    fn as_weak_ref(&self) -> WeakRef<gtk::Widget> {
        self.as_ref().downgrade()
    }
}

pub trait WindowComponent {
    fn present(&self);

    fn destroy(&self);
}

pub trait UpdateableComponent: 'static {
    type Message: Clone + 'static;

    fn update(&self, message: Self::Message);
}
