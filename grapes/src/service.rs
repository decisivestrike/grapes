use tokio::sync::{RwLockReadGuard, broadcast};

pub trait Cacheable: Service {
    fn cache() -> RwLockReadGuard<'static, Self::Message>;

    #[allow(async_fn_in_trait)]
    async fn cache_async() -> RwLockReadGuard<'static, Self::Message>;

    fn cache_copy() -> Self::Message
    where
        Self::Message: Copy;

    #[allow(async_fn_in_trait)]
    async fn cache_copy_async() -> Self::Message
    where
        Self::Message: Copy;
}

pub trait Service {
    type Message: Clone + 'static;
}

pub trait Broadcast: Service {
    fn subscribe() -> broadcast::Receiver<Self::Message>;
}
