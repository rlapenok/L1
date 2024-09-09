use std::sync::Arc;

use crate::worker::Worker;
use flume::{bounded, Sender};
use tokio::sync::Mutex;

pub struct Pool {
    shutdown_senders: Arc<Mutex<Vec<Sender<()>>>>,
}

impl Pool {
    pub fn run(size: usize) -> Self {
        let mut shutdown_senders = Vec::with_capacity(size);
        for i in 1..=size {
            let (sender, receiver) = bounded(size);
            Worker::new(i, receiver);
            shutdown_senders.push(sender);
        }
        let shutdown_senders = Arc::new(Mutex::new(shutdown_senders));
        Self { shutdown_senders }
    }
    pub async fn shutdown(&self) {
        let guard = &mut *self.shutdown_senders.lock().await;
        while let Some(sender) = guard.pop() {
            if let Err(err) = sender.send_async(()).await {
                eprintln!("Error while shutdown Worker:{}", err)
            }
        }
    }
}
