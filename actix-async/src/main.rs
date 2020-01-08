use actix_async::*;
use std::sync::{Arc, Mutex};
#[actix_rt::main]
async fn main() {
    let (_, receiver) = tokio::sync::mpsc::channel(1);
    let receiver = Arc::new(Mutex::new(receiver));
    tokio::spawn(async { ServiceController::new(receiver).run().await; });
}
