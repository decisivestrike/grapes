pub trait Updateable: 'static {
    type Message: Clone + 'static;

    fn update(&self, message: Self::Message);
}
