pub mod receiver;

// Import common modules below.
use crate::protos;
use crate::types::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Phantasm {
    connections: Mutex<HashMap<ConnectionID, UnboundedSender<Message>>>,
}

impl Phantasm {
    pub fn open() -> Result<Self, Box<dyn Error>> {
        Self::setup_dir()?;
        Ok(Phantasm { connections: Mutex::new(HashMap::new()) })
    }

    pub fn add_connection(
        &self,
        id: &ConnectionID,
        sender: &UnboundedSender<Message>,
    ) {
        let mut conn = self.connections.lock().unwrap();
        conn.insert(*id, sender.to_owned());
    }

    pub fn remove_connection(&self, id: &ConnectionID) {
        let mut conn = self.connections.lock().unwrap();
        conn.remove(id);
    }

    fn dir() -> PathBuf {
        match env::var("PHANTASM_DIR") {
            Ok(dir) => PathBuf::from(dir),
            Err(_) => PathBuf::from("phantasm"),
        }
    }

    fn setup_dir() -> Result<(), Box<dyn Error>> {
        let dir = Self::dir();
        std::fs::create_dir_all(&dir)?;
        Ok(())
    }
}
