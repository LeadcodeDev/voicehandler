use anywho::Error;
use tokio::sync::watch::{Receiver, Sender, channel};

#[derive(Clone)]
pub struct Reactive<T>
where
    T: Clone + Send + Sync + 'static,
{
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Reactive<T>
where
    T: Clone + Send + Sync,
{
    pub fn new(value: T) -> Self {
        let (tx, rx) = channel(value);
        Self { tx, rx }
    }

    pub async fn set(&self, value: T) -> Result<(), Error> {
        self.tx.send(value).map_err(|err| Error::from(err))
    }

    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    pub async fn changed(&mut self) -> Result<(), Error> {
        self.rx.changed().await.map_err(|err| Error::from(err))
    }
}
