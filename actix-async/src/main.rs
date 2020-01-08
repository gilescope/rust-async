use actix_async::*;
use actix_web::HttpServer;
use std::sync::{Arc, Mutex};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (_sender, receiver) = tokio::sync::mpsc::channel(10);
    let receiver = Arc::new(Mutex::new(receiver));
    tokio::spawn(async {
        let mut service_controller = ServiceController::new(receiver);
        service_controller.run().await;
    });
    Ok(())
}
