use std::time::Duration;

use flume::Receiver;
use tokio::time::interval;

pub struct Worker();
impl Worker {
    pub fn new(id: usize, message_receiver: Receiver<()>) -> Self {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(id as u64));
            loop {
                tokio::select! {
                    _ = message_receiver.recv_async()=>{
                        println!("Worker № {} shutdown successfully",id);
                        break;
                    }
                    _ = interval.tick() => {

                        println!("Worker № {} is working",id)

                    }
                };
            }
        });
        Self {}
    }
}
