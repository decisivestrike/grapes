use gtk::glib::clone;
use gtk::prelude::*;

use crate::{State, derived, effect::effect};

pub trait Reactive<T> {
    fn reactive(initial: &State<T>) -> Self;

    fn with_effect<F>(e: F) -> Self
    where
        Self: Sized,
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let derived = derived(e);

        Self::reactive(&derived)
    }
}

impl<T> Reactive<T> for gtk::Button
where
    T: ToString + 'static,
{
    fn reactive(label: &State<T>) -> Self {
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
    fn reactive(label: &State<T>) -> Self {
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
