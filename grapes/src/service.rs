use gtk::glib::{self, clone};
use tokio::sync::broadcast;

use crate::updateable::Updateable;

pub trait CachedService<T>: Service<T>
where
    T: Clone + 'static,
{
    fn cache(&self) -> &T;
}

pub trait Service<T>
where
    T: Clone + 'static,
{
    fn subscribe() -> broadcast::Receiver<T>;
}

pub trait Connectable: Updateable {
    fn connect_service<T>(&self)
    where
        T: Service<Self::Message>;
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
                        updateable.update(message.into());
                    }
                }
            }
        ));
    }
}
