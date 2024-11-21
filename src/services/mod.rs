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

#[derive(Debug, Clone)]
pub struct Configuration {
    pub auto_approve: bool,
}

#[cfg(test)]
impl Default for Configuration {
    fn default() -> Self {
        Configuration { auto_approve: false }
    }
}

#[derive(Debug)]
pub struct Phantasm {
    config: Configuration,
    connections: Mutex<HashMap<ConnectionID, Connection>>,
    approvals: Mutex<HashMap<ApprovalID, Sender<ApprovalResponse>>>,
}

impl Phantasm {
    pub fn open(config: &Configuration) -> Result<Self, Box<dyn Error>> {
        Ok(Phantasm {
            config: config.clone(),
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

        // Send the approval response via the oneshot sender to the oneshot
        // receiver to complete the approval request.
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

    /// Get the connection with the smallest load.
    ///
    /// Load refers to the number of approval requests that are currently
    /// being processed by the connection. This method is used to distribute
    /// the approval requests somewhat evenly among the connections.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_receive_message() {
        let config = Configuration::default();
        let phantasm = Phantasm::open(&config).unwrap();

        // Simulate queueing a pending approval request.
        let approval_id = ApprovalID::new();
        let (sender, _) = tokio::sync::oneshot::channel();
        phantasm.add_approval(approval_id, sender);

        // Simulate receiving an approval response message.
        let params = json!({ "key": "value" });
        let response = ApprovalResponse {
            id: approval_id,
            approved: true,
            parameters: params.to_string(),
            approver: Approver {
                name: "Alice".to_string(),
                email: "alice@thewonderland.com".to_string(),
            },
        };

        let response_string = serde_json::to_string(&response).unwrap();
        let message = Message::Text(response_string);
        phantasm.receive_message(&message).await;

        // There should be no pending approvals after receiving the message
        // since the pending approval will be completed and removed.
        let approvals = phantasm.approvals.lock().unwrap();
        assert!(approvals.is_empty());
    }
}
