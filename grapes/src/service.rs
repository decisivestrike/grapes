use tokio::sync::broadcast;

pub trait Service {
    type Message: Clone + 'static;
}

pub trait Broadcast: Service {
    fn subscribe() -> broadcast::Receiver<Self::Message>;
}
