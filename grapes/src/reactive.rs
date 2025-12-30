use gtk::glib::clone;
use gtk::prelude::*;

use crate::{State, derived, effect};

pub trait Reactive<T> {
    fn statefull(initial: &State<T>) -> Self;

    fn derived<F>(e: F) -> Self
    where
        Self: Sized,
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let derived = derived(e);

        Self::statefull(&derived)
    }
}

impl<T> Reactive<T> for gtk::Button
where
    T: ToString + 'static,
{
    fn statefull(label: &State<T>) -> Self {
        let button = gtk::Button::new();

        effect(clone!(
            #[strong]
            button,
            #[strong]
            label,
            move || button.set_label(&label.get().to_string())
        ));

        button
    }
}

impl<T> Reactive<T> for gtk::Label
where
    T: ToString + 'static,
{
    fn statefull(label: &State<T>) -> Self {
        let button = gtk::Label::new(Some(&label.get().to_string()));

        effect(clone!(
            #[strong]
            button,
            #[strong]
            label,
            move || button.set_label(&label.get().to_string())
        ));

        button
    }
}
