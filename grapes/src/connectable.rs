use gtk::glib::{self, clone};

use crate::{Service, updateable::Updateable};

pub trait Connectable: Updateable {
    fn connect_service<T>(&self)
    where
        T: Service<Self::Message>;

    fn connect_service_unmatched<W, T, F>(&self, matcher: F)
    where
        W: Clone + 'static,
        T: Service<W>,
        F: Fn(W) -> Self::Message + 'static;
}

impl<C> Connectable for C
where
    C: Updateable,
{
    fn connect_service<T>(&self)
    where
        T: Service<Self::Message>,
    {
        let mut rx = T::subscribe();

        glib::spawn_future_local(clone!(
            #[strong(rename_to=updateable)]
            self,
            async move {
                loop {
                    if let Ok(message) = rx.recv().await {
                        updateable.update(message);
                    }
                }
            }
        ));
    }

    fn connect_service_unmatched<W, T, F>(&self, matcher: F)
    where
        W: Clone + 'static,
        T: Service<W>,
        F: Fn(W) -> Self::Message + 'static,
    {
        let mut rx = T::subscribe();

        glib::spawn_future_local(clone!(
            #[strong(rename_to=updateable)]
            self,
            async move {
                loop {
                    if let Ok(message) = rx.recv().await {
                        updateable.update(matcher(message));
                    }
                }
            }
        ));
    }
}
