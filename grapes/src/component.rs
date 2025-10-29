use crate::Service;
use gtk::glib::{self, clone};

pub trait GtkCompatible: AsRef<gtk::Widget> + Clone + 'static {
    fn as_widget_ref(&self) -> &gtk::Widget;
}

pub trait Component: GtkCompatible {
    type Message: Clone + 'static;
    type Props;

    fn new(props: Self::Props) -> Self;

    fn update(&self, message: Self::Message);

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
