use std::rc::Rc;

use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;

use crate::{State, derived, effect};

/// Allows you to bind a state to a gtk widget
pub trait Reactive<T> {
    fn statefull(initial: &Rc<State<T>>) -> Self;

    fn derived<F>(f: F) -> Self
    where
        Self: Sized,
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let derived = derived(f);

        Self::statefull(&derived)
    }
}

impl<T> Reactive<T> for gtk::Button
where
    T: ToString + 'static,
{
    fn statefull(label: &Rc<State<T>>) -> Self {
        let button = gtk::Button::new();

        effect(clone!(
            #[strong]
            button,
            #[weak]
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
    fn statefull(label: &Rc<State<T>>) -> Self {
        let button = gtk::Label::new(None);

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
