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





// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG","debug,actix_async=trace");
    env_logger::init();

    let (sender, receiver) = sync::mpsc::channel();

    tokio::spawn(async move {
        loop {
            let message = receiver.recv().unwrap();
            match message {
                Message::RunCheck => {
                    info!("now should be able to run task");
                },
                Message::Terminate => {
                    break; // loop
                },
            }
        }
    });


    //let mut controller = Controller::new(receiver);
    //let mut controllerarc = Arc::new(Mutex::new(Controller::new()));
    

    // let mut controllerarc = Arc::clone(&controllerarc);
    tokio::spawn(async move {
        //controller.run().await;
        
        // let mut controllerarc = controllerarc.lock().unwrap();
        // controllerarc.run().await;
    });
    
    println!("Starting web server");
    
    let sender = Arc::new(Mutex::new(sender));

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

    res
}