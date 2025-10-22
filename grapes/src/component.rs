use std::{fmt, hash};

use crate::Service;
use gtk::{
    Widget,
    glib::{self, clone, object::IsA},
};

pub trait GtkCompatible:
    Clone + fmt::Debug + Default + hash::Hash + PartialEq + PartialOrd + Eq + Ord + 'static
{
    type Root: IsA<Widget>;

    fn root(&self) -> Self::Root;

    fn as_widget_ref(&self) -> &gtk::Widget;
}

pub trait Component: GtkCompatible {
    type Message: Clone + 'static;

    fn update(&self, message: Self::Message);

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
