use actix_web::{get, App, HttpServer, HttpResponse, Responder, Error as ActixError, web};

use std::time::Duration;
use tokio::{time, process::Command};

use std::sync::{Arc, RwLock, Mutex, MutexGuard};
use std::*;

#[macro_use]
extern crate log;

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

#[derive(Debug)]
enum Message {
    RunCheck,
    Terminate,
}

#[derive(Debug)]
struct Controller {
    receiver: sync::mpsc::Receiver<Message>,
    counter: std::sync::Mutex<i32>,
    number: i32,
    running: bool,
    check: std::sync::Mutex<Check>,
}

impl Controller {
    pub fn new(receiver: sync::mpsc::Receiver<Message>) -> Self {
        Controller {
            receiver,
            counter: Mutex::new(0),
            number: 0,
            running: false,
            check: Mutex::new(Check::default()),
        }
    }
    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            info!("Running async method in Controller");
            Command::new("date").spawn()?.await?;
            self.update();
        } 
    }
    pub fn update(&mut self) -> Result<(), std::io::Error> {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        info!("Counter is:{}", counter);
        self.running = true;
        Ok(())
    }
}

/// Prints output to 
async fn dating() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        // Ensures delay between calls -> first is executed immediatelly
        // Next is excuted after specified duration
        interval.tick().await;

        info!("Running async command");
        Command::new("date").spawn()?.await?;
    }
}


#[derive(Debug)]
struct ServiceController {
    receiver: Arc<Mutex<sync::mpsc::Receiver<Message>>>,
    //sender: Arc<Mutex<sync::mpsc::Sender<Message>>>,
}

impl ServiceController {
    pub fn new(receiver: Arc<Mutex<sync::mpsc::Receiver<Message>>>) -> Self {
    // pub fn new(receiver: sync::mpsc::Receiver<Message>, sender: Arc<Mutex<sync::mpsc::Sender<Message>>>) -> Self {
    //pub fn new(sender: Arc<Mutex<sync::mpsc::Sender<Message>>>) -> Self {
        ServiceController {
            receiver,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let receiver = Arc::clone(&self.receiver);
        // let receiver = self.
        std::thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                trace!("message received {:?}", &message);
                match message {
                    Message::RunCheck => {
                        info!("now should be able to run task");
                    },
                    Message::Terminate => {
                        break; // loop
                    },
                }
            }
            trace!("tokio loop finishes");
        });

        Ok(())
    }
}

impl Drop for ServiceController {
    fn drop(&mut self) {
        trace!("dropping service controller");
        // self.sender.lock().unwrap().send(Message::Terminate).unwrap();
    }
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
                    info!("now should be able to run task");
                },
                Message::Terminate => {
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