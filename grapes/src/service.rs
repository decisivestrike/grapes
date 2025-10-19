use gtk::{
    Widget,
    glib::{self, clone, object::IsA},
};
use tokio::sync::broadcast;

use crate::State;

pub trait Service<T>
where
    T: Clone + 'static,
{
    fn subscribe() -> broadcast::Receiver<T>;

    fn connect_widget<W, F>(widget: W, on_message: F)
    where
        W: IsA<Widget>,
        F: Fn(&W, T) + 'static,
    {
        let mut rx = Self::subscribe();

        glib::spawn_future_local(clone!(
            #[strong]
            widget,
            async move {
                loop {
                    if let Ok(message) = rx.recv().await {
                        on_message(&widget, message);
                    }
                }
            }
        ));
    }

    fn connect_state(state: State<T>) {
        let mut rx = Self::subscribe();

        glib::spawn_future_local(clone!(
            #[strong]
            state,
            async move {
                loop {
                    if let Ok(message) = rx.recv().await {
                        state.set(message)
                    }
                }
            }
        ));
    }

    fn connect_state_with<F, U>(state: State<U>, on_message: F)
    where
        F: Fn(&State<U>, T) + 'static,
        U: 'static,
    {
        let mut rx = Self::subscribe();

        glib::spawn_future_local(clone!(
            #[strong]
            state,
            async move {
                loop {
                    if let Ok(message) = rx.recv().await {
                        on_message(&state, message);
                    }
                }
            }
        ));
    }
}
