use actix_async::ServiceController;
use std::sync::{Arc, Mutex};

fn main() {
    let (_, receiver) = tokio::sync::mpsc::channel(1);
    let receiver = Arc::new(Mutex::new(receiver));
    tokio::spawn(async { ServiceController::new(receiver).run().await; });
}
