use crate::{Broadcast, updateable::Updateable};
use gtk::glib::{self, clone};

pub trait Connectable: Updateable {
    fn connect_service<S>(&self)
    where
        S: Broadcast<Message = Self::Message>;

    fn connect_service_unmatched<S>(
        &self,
        matcher: impl Fn(S::Message) -> Self::Message + 'static,
    ) where
        S: Broadcast;
}

impl<C> Connectable for C
where
    C: Updateable + Clone,
{
    fn connect_service<S>(&self)
    where
        S: Broadcast<Message = Self::Message>,
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

    fn connect_service_unmatched<S>(
        &self,
        matcher: impl Fn(S::Message) -> Self::Message + 'static,
    ) where
        S: Broadcast,
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
