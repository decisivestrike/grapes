use tokio::sync::broadcast;

pub trait CachedService: Service {
    fn cache(&self) -> &Self::Message;
}

pub trait Service {
    type Message: Clone + 'static;

    fn subscribe() -> broadcast::Receiver<Self::Message>;
}
