mod receiver;

// Import common modules below.
use crate::protos;
use crate::types::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot::Sender;
use tokio_tungstenite::tungstenite::Message;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct Phantasm {
    connections: Mutex<HashMap<ConnectionID, Connection>>,
    approvals: Mutex<HashMap<ApprovalID, Sender<ApprovalResponse>>>,
}

impl Phantasm {
    pub fn open() -> Result<Self, Box<dyn Error>> {
        Ok(Phantasm {
            connections: Mutex::new(HashMap::new()),
            approvals: Mutex::new(HashMap::new()),
        })
    }

    pub async fn receive_message(&self, message: &Message) {
        let message = match message {
            Message::Text(text) => text,
            _ => return,
        };

        let message: ApprovalResponse = match serde_json::from_str(message) {
            Ok(message) => message,
            Err(error) => {
                tracing::error!("Failed to parse the message: {error}");
                return;
            },
        };

        // Relay the approval response to the oneshot sender to complete
        // the approval request.
        let approval_id = message.id;
        if let Some(sender) = self.remove_approval(&approval_id) {
            let _ = sender.send(message);
        }
    }

    pub fn add_connection(&self, id: ConnectionID, connection: Connection) {
        let mut connections = self.connections.lock().unwrap();
        connections.insert(id, connection);
    }

    pub fn remove_connection(&self, id: &ConnectionID) {
        let mut connections = self.connections.lock().unwrap();
        connections.remove(id);
    }

    fn get_connection(&self, id: &ConnectionID) -> Option<Connection> {
        let connections = self.connections.lock().unwrap();
        connections.get(id).cloned()
    }

    fn get_lightest_connection(&self) -> Option<ConnectionID> {
        let connections = self.connections.lock().unwrap();
        connections
            .iter()
            .min_by_key(|(_, connection)| connection.load)
            .map(|(id, _)| *id)
    }

    fn add_load(&self, id: &ConnectionID) {
        let mut connections = self.connections.lock().unwrap();
        if let Some(connection) = connections.get_mut(id) {
            connection.load += 1;
        }
    }

    fn reduce_load(&self, id: &ConnectionID) {
        let mut connections = self.connections.lock().unwrap();
        if let Some(connection) = connections.get_mut(id) {
            if connection.load > 0 {
                connection.load -= 1;
            }
        }
    }

    fn add_approval(&self, id: ApprovalID, sender: Sender<ApprovalResponse>) {
        let mut approvals = self.approvals.lock().unwrap();
        approvals.insert(id, sender);
    }

    fn remove_approval(
        &self,
        id: &ApprovalID,
    ) -> Option<Sender<ApprovalResponse>> {
        let mut approvals = self.approvals.lock().unwrap();
        approvals.remove(id)
    }
}
