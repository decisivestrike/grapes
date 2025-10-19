use crate::Service;
use gtk::{
    Widget,
    glib::{self, clone, object::IsA},
};

pub trait Component: Clone + 'static {
    type Root: IsA<Widget>;
    type Message: Clone + 'static;

    fn update(&self, message: Self::Message);

    fn view(&self) -> Self::Root;

    fn connect<T>(&self)
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
