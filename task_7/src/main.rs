use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
        Arc,
    },
    time::{Duration, Instant},
};

use tokio::{
    spawn,
    task::JoinSet,
    time::{interval, sleep},
};
use tokio_util::sync::CancellationToken;

async fn cancellation_token() {
    let token = CancellationToken::new();
    let mut join_set = JoinSet::new();
    for i in 1..=10 {
        let token = token.child_token();
        let mut interval = interval(Duration::from_secs(1));
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = token.cancelled()=>{
                            println!("Task № {} cancelled",i);
                            sleep(Duration::from_secs(1)).await;
                            break;
                    }

                    _ = interval.tick()=>{
                        println!("Task № {} tick",i)
                    }


                };
            }
        });
        join_set.spawn(task);
    }
    let task = spawn(async move {
        sleep(Duration::from_secs(5)).await;
        token.cancel();
        println!("Cancellation task done")
    });
    join_set.spawn(task);
    join_set.join_all().await;
}

async fn atomic_flag() {
    let atomic = Arc::new(AtomicBool::new(false));
    let mut join_set = JoinSet::new();
    for i in 1..=10 {
        let mut interval = interval(Duration::from_secs(1));
        let atomic = atomic.clone();
        let task = tokio::spawn(async move {
            loop {
                interval.tick().await;
                if !atomic.load(Ordering::Relaxed) {
                    println!("Task № {} tick", i);
                    continue;
                }
                println!("Task № {} cancelled", i);
                break;
            }
        });
        join_set.spawn(task);
    }

    let task = spawn(async move {
        sleep(Duration::from_secs(10)).await;
        atomic.store(true, Ordering::Relaxed);
        println!("Cancellation task done")
    });

    join_set.spawn(task);
    join_set.join_all().await;
}

async fn drop_channel() {
    let mut join_set = JoinSet::new();
    let mut senders = Vec::new();
    for i in 1..=10 {
        let (sender, receiver) = channel::<()>();
        senders.push(sender);
        let task = spawn(async move {
            receiver
                .into_iter()
                .for_each(|_| println!("Worker № {} recv msg", i));
        });
        join_set.spawn(task);
    }
    let task = spawn(async move {
        let instant = Instant::now();
        while instant.elapsed() < Duration::from_secs(2) {
            senders.iter().for_each(|sender| {
                if let Err(err) = sender.send(()) {
                    eprintln!("Error while send message {}", err)
                }
                println!("Message send successfully")
            });
        }
        drop(senders);
    });
    join_set.spawn(task);
    join_set.join_all().await;
}

#[tokio::main]
async fn main() {
    let mut join_set = JoinSet::new();
    join_set.spawn(cancellation_token());
    join_set.spawn(atomic_flag());
    join_set.spawn(drop_channel());
    join_set.join_all().await;
}
