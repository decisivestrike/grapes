use crate::RT;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::{task::JoinHandle, time::sleep};

pub struct Interval {
    handle: JoinHandle<()>,
}

impl Interval {
    pub fn set<F>(f: F, delay: Duration) -> Self
    where
        F: Fn() + Send + 'static,
    {
        let handle = RT.spawn(async move {
            loop {
                f();
                sleep(delay).await;
            }
        });

        Self { handle }
    }

    pub fn clear(self) {
        self.handle.abort();
    }
}

pub struct Timeout {
    handle: JoinHandle<()>,
}

impl Timeout {
    pub fn set<F>(f: F, delay: Duration) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let handle = RT.spawn(async move {
            sleep(delay).await;
            f();
        });

        Self { handle }
    }

    pub fn clear(self) {
        self.handle.abort();
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.handle.is_finished() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{Arc, RwLock},
        time::Duration,
    };

    #[tokio::test]
    async fn timeout_set() {
        tokio::time::pause();

        let count = Arc::new(RwLock::new(0));
        let duration = Duration::from_secs(1);

        let count_clone = count.clone();
        let timeout = Timeout::set(
            move || {
                let mut count = count_clone.write().unwrap();
                *count += 1;
            },
            duration,
        );

        tokio::time::advance(duration).await;
        timeout.await;

        let result = *count.read().unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn timeout_cancel() {
        tokio::time::pause();

        let count = Arc::new(RwLock::new(0));
        let duration = Duration::from_secs(1);

        let count_clone = count.clone();
        let timeout = Timeout::set(
            move || {
                let mut count = count_clone.write().unwrap();
                *count += 1;
            },
            duration,
        );

        tokio::time::advance(duration / 2).await;
        timeout.clear();
        tokio::time::advance(duration / 2).await;

        let result = *count.read().unwrap();
        assert_eq!(result, 0);
    }
}
