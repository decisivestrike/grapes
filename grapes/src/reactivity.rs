use gtk::glib::clone;
use gtk::prelude::*;

use crate::{State, effect::effect};

pub trait Reactive<T> {
    fn reactive(initial: &State<T>) -> Self;
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
