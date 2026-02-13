use grapes::{Component, State, gtk};
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
        Self {
            label: gtk::Label::new(None),
            state: state.clone(),
        }
    }

    pub fn state(&self) -> Rc<State<T>> {
        self.state.clone()
    }
}
