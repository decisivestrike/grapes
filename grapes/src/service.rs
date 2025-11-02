use tokio::sync::broadcast;

pub trait CachedService<T>: Service<T>
where
    T: Clone + 'static,
{
    fn cache(&self) -> &T;
}

pub trait Service<T>
where
    T: Clone + 'static,
{
    fn subscribe() -> broadcast::Receiver<T>;
}
