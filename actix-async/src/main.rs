use actix_web::{get, App, HttpServer, Responder, web};

use std::time::Duration;
use tokio::{task, time, process::Command};

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
struct Controller {
    counter: std::sync::Mutex<i32>,
    // number: i32,
}

impl Controller {
    pub async fn run(&mut self) -> Result<(), std::io::Error> {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            info!("Running command");
            Command::new("date").spawn()?.await?;
        } 
    }
}

/// Prints output to 
async fn dating() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        info!("Running command");
        Command::new("date").spawn()?.await?;
    }
}

// #[tokio::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG","debug,actix-async=trace");
    env_logger::init();
    task::spawn(dating());

    let mut control = Controller::default();
    task::spawn(async move {
        control.run().await
    });

    println!("Starting web server");

// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::new(|| App::new().service(index_id_name).service(index))
        .bind("127.0.0.1:8080")?
        .start()
        .await
}