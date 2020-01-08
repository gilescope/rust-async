use actix_async::*;
use actix_web::HttpServer;
use std::sync::{Arc, Mutex};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (_sender, receiver) = tokio::sync::mpsc::channel(10);
    let receiver = Arc::new(Mutex::new(receiver));
    let receiver_tokio2 = Arc::clone(&receiver);
    let mut service_controller
        = ServiceController::new(Arc::clone(&receiver));
    service_controller.run().await.unwrap();
    tokio::spawn(async move {
        let mut service_controller
            = ServiceController::new(receiver_tokio2);
        service_controller.run().await;
    });
    let res = HttpServer::new(move || unimplemented!())
        .bind("")?
        .run()
        .await;
    res
}

