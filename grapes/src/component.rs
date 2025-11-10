use crate::Service;
use gtk::{
    Application,
    gdk::Monitor,
    glib::{self, clone},
};

pub trait GtkCompatible: AsRef<gtk::Widget> + Clone + 'static {
    fn as_widget_ref(&self) -> &gtk::Widget;
}

pub trait Component: GtkCompatible {
    const NAME: &str;

    type Message: Clone + 'static;
    type Props;

    fn new(props: Self::Props) -> Self;

    fn update(&self, message: Self::Message);

    fn name(&self) -> &str {
        Self::NAME
    }

    fn connect_service<T>(&self)
    where
        T: Service<Self::Message>,
    {
        let mut rx = T::subscribe();

        glib::spawn_future_local(clone!(
            #[strong(rename_to=component)]
            self,
            async move {
                loop {
                    if let Ok(message) = rx.recv().await {
                        component.update(message.into());
                    }
                }
            }
        ));
    }
}

pub trait WindowComponent {
    type Props;

    fn new(app: &Application, monitor: &Monitor, props: Self::Props) -> Self;

    fn present(&self);
}
