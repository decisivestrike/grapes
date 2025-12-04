use gtk::glib::{self, clone};

use crate::{Service, updateable::Updateable};

pub trait Connectable: Updateable {
    fn connect_service<S>(&self)
    where
        S: Service<Message = Self::Message>;

    fn connect_service_unmatched<S, F>(&self, matcher: F)
    where
        S: Service,
        F: Fn(S::Message) -> Self::Message + 'static;
}

impl<C> Connectable for C
where
    C: Updateable,
{
    fn connect_service<S>(&self)
    where
        S: Service<Message = Self::Message>,
    {
        let mut rx = S::subscribe();

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

    fn connect_service_unmatched<S, F>(&self, matcher: F)
    where
        S: Service,
        F: Fn(S::Message) -> Self::Message + 'static,
    {
        let mut rx = S::subscribe();

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
