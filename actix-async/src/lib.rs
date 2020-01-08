use std::sync::{Arc, Mutex};
use std::*;

pub struct ServiceController {
    receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<String>>>,
}

impl ServiceController {
    pub fn new(receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<String>>>) -> Self {
        ServiceController { receiver }
    }

    pub async fn run(&mut self) -> Result<(), ()> {
        let receiver = Arc::clone(&self.receiver);
        while let Some(message) = receiver.lock().unwrap().recv().await {}
        Ok(())
    }
}
