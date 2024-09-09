use std::sync::Arc;

use pool::Pool;
use tokio::{signal::ctrl_c, spawn};

mod pool;
mod worker;

#[tokio::main]
async fn main() {
    let pool = Arc::new(Pool::run(5));
    let poll_shutdown = pool.clone();

    spawn(async move {
        ctrl_c().await.expect("failed to install Ctrl+C handler");
        poll_shutdown.shutdown().await;
    })
    .await
    .expect("Shutdown error");
}
