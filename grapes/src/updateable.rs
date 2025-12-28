use crate::{State, local::LocalState};

pub trait Updateable: 'static {
    type Message: Clone + 'static;

    fn update(&self, message: Self::Message);
}

impl<T: Clone + 'static> Updateable for LocalState<T> {
    type Message = T;

    fn update(&self, message: T) {
        self.set(message);
    }
}
