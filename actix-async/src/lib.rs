use std::sync::{Arc, Mutex};
use std::*;

#[macro_use]
extern crate log;

#[derive(Debug)]
pub enum Message {
    RunCheck,
    Terminate,
}

#[derive(Debug)]
pub struct ServiceController {
    receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>,
    //sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>,
}

impl ServiceController {
    pub fn new(receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>) -> Self {
        // pub fn new(receiver: tokio::sync::mpsc::Receiver<Message>, sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>) -> Self {
        //pub fn new(sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>) -> Self {
        ServiceController { receiver }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let receiver = Arc::clone(&self.receiver);
        // let receiver = self.
        // tokio::spawn(async move {
        while let Some(message) = receiver.lock().unwrap().recv().await {
            // let message = receiver.lock().unwrap().recv().await.unwrap();
            trace!("ServiceController: message received {:?}", &message);
            match message {
                Message::RunCheck => {
                    test();
                    info!("ServiceController: now should be able to run task");
                }
                Message::Terminate => {
                    info!("ServiceController: now terminating project");
                    break; // loop
                }
            }
        }
        // trace!("ServiceController: tokio loop finishes");
        // });

        Ok(())
    }
}

impl Drop for ServiceController {
    fn drop(&mut self) {
        trace!("dropping service controller");
        // self.sender.lock().unwrap().send(Message::Terminate).unwrap();
    }
}

fn test() {
    info!("test function");
}
