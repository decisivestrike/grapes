use grapes::{Component, State, effect, glib::clone::Downgrade, gtk};
use std::rc::Rc;

#[derive(Debug, Component)]
pub struct StatefullLabel<T>
where
    T: 'static,
{
    #[root]
    label: gtk::Label,
    state: Rc<State<T>>,
}

impl<T> StatefullLabel<T>
where
    T: AsRef<str>,
{
    pub fn new(state: &Rc<State<T>>) -> StatefullLabel<T> {
        let label = gtk::Label::new(None);

        {
            let weak_state = state.downgrade();
            let weak_label = label.downgrade();

            effect(move || {
                if let Some(state) = weak_state.upgrade()
                    && let Some(label) = weak_label.upgrade()
                {
                    label.set_label(state.get().as_ref());
                }
            });
        }

        Self {
            label,
            state: state.clone(),
        }
    }

    pub fn state(&self) -> Rc<State<T>> {
        self.state.clone()
    }
}
