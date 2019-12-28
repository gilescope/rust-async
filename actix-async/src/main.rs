use std::*;
use actix_web::{get, App, HttpServer, HttpResponse, Responder, Error as ActixError, web};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate log;

use lib::*;

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
}

#[get("/api/stop")]
async fn api_stop(
    sender: web::Data<Arc<Mutex<sync::mpsc::Sender<Message>>>>,
) -> Result<HttpResponse, ActixError> {
    trace!("{:?}", sender);
    let sender = sender.lock().unwrap();
    let err = sender.send(Message::RunCheck).unwrap();
    // if let Err(_) = sender.send(Message::RunCheck).await {
    //     error!("Not possible to send message -> RunCheck");
    // }
    let text = format!("Shoud stop task\n");
    Ok(HttpResponse::Ok().body(text))
}



#[get("/index.html")]
async fn index() -> &'static str {
    "hello\n"
}

#[derive(Default, Debug)]
struct Check {
    ip: String,
    port: String,
}

// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG","debug,actix_async=trace");
    env_logger::init();
    
    // Create sender and receiver to communicate with loop
    let (sender, receiver) = sync::mpsc::channel();
    let sender = Arc::new(Mutex::new(sender)); // <-- Actix loop
    let sender_exit = Arc::clone(&sender); // <-- Ctrl+C handler
    let receiver = Arc::new(Mutex::new(receiver));

    // Gracefull shutdown -> SIGTERM received -> send message terminate
    ctrlc::set_handler(move || {
        // Two loops -> two messages
        sender_exit.lock().unwrap().send(Message::Terminate)
        .expect("Not possible to send terminate message");
        sender_exit.lock().unwrap().send(Message::Terminate)
        .expect("Not possible to send terminate message");
    }).expect("Error setting Ctrl+C handler");

    let mut service_controller = ServiceController::new(Arc::clone(&receiver));
    service_controller.run().expect("Not possible to run thread loop");

    tokio::spawn(async move {
        loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            trace!("message received {:?}", &message);
            match message {
                Message::RunCheck => {
                    info!("separtate tokio::spawn: now should be able to run task");
                },
                Message::Terminate => {
                    info!("separtate tokio::spawn: now terminating project");
                    break; // loop
                },
            }
        }
        trace!("tokio loop finishes");
    });
    
    info!("Starting web server");
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = HttpServer::new( move || 
        App::new()
            .service(index_id_name)
            .service(index)
            .service(api_stop)
            .data(Arc::clone(&sender))
        )
        .bind("127.0.0.1:8080")?
        .start()
        .await;

    info!("Server finished");
    res
}