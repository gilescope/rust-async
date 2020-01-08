use actix_web::{get, web, App, Error as ActixError, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use actix_async::*;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = tokio::sync::mpsc::channel(10);
    let sender = Arc::new(Mutex::new(sender));
    let sender_exit = Arc::clone(&sender);
    let receiver = Arc::new(Mutex::new(receiver));
    let receiver_tokio = Arc::clone(&receiver);
    let receiver_tokio2 = Arc::clone(&receiver);
    let receiver_tokio3 = Arc::clone(&receiver);
    let mut service_controller = ServiceController::new(Arc::clone(&receiver));
    service_controller
        .run()
        .await
        .expect("Not possible to run thread loop");
    tokio::spawn(async move { unimplemented!() });
    tokio::spawn(async move {
        let mut service_controller = ServiceController::new(receiver_tokio2);
        service_controller
            .run()
            .await
            .expect("Not possible to run thread loop");
    });
    //info!("Starting web server");
    let res = HttpServer::new(move || unimplemented!())
        .bind("127.0.0.1:8080")?
        .run()
        .await;
    //info!("Server finished");
    res
}