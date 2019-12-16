use actix_web::{get, App, HttpServer, Responder, web};

use std::time::Duration;
use tokio::{task, time, process::Command};

use std::sync::{Arc, RwLock, Mutex, MutexGuard};

#[macro_use]
extern crate log;

#[get("/{id}/{name}/index.html")]
async fn index_id_name(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", info.1, info.0)
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

#[derive(Default, Debug)]
struct Controller {
    counter: std::sync::Mutex<i32>,
    number: i32,
    running: bool,
    check: std::sync::Mutex<Check>,
}

impl Controller {
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
    std::env::set_var("RUST_LOG","debug,actix-async=trace");
    env_logger::init();

    let mut controller = Controller::default();
    let mut controllerarc = Arc::new(Mutex::new(Controller::default()));
    

    // let mut controllerarc = Arc::clone(&controllerarc);
    tokio::spawn(async move {
        controller.run().await;
        
        // let mut controllerarc = controllerarc.lock().unwrap();
        // controllerarc.run().await;
    });
    
    println!("Starting web server");
    
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = HttpServer::new( move || 
            App::new()
                .service(index_id_name)
                .service(index)
                .data(Arc::clone(&controllerarc))
        )
        .bind("127.0.0.1:8080")?
        .start()
        .await;

    res
}