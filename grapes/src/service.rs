use tokio::sync::broadcast;

pub trait Cacheable: Service {
    fn cache(&self) -> &Self::Message;
}

pub trait Service {
    type Message: Clone + 'static;
}

pub trait Broadcast: Service {
    fn subscribe() -> broadcast::Receiver<Self::Message>;
}
