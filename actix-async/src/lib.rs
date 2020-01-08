use std::sync::{Arc, Mutex};
use std::*;

#[derive(Debug)]
pub enum Message {
    RunCheck,
    Terminate,
}

#[derive(Debug)]
pub struct ServiceController {
    receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>,
}

impl ServiceController {
    pub fn new(receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>) -> Self {
        ServiceController { receiver }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let receiver = Arc::clone(&self.receiver);
        while let Some(message) = receiver.lock().unwrap().recv().await {
        }
        Ok(())
    }
}