use crate::RT;
use std::time::Duration;
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
