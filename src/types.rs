use crate::protos;
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionID(Uuid);

impl ConnectionID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub sender: UnboundedSender<Message>,
    pub load: usize,
}

impl Connection {
    pub fn new(sender: UnboundedSender<Message>) -> Self {
        Self { sender, load: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct ApprovalID(Uuid);

impl ApprovalID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ApprovalID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// The types below are used in dashboard-coordinator communication.
// They are serialized into JSON strings before being sent as WebSocket
// messages. If you change these types, make sure to update the types in
// the dashboard as well.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: ApprovalID,
    pub name: String,
    pub parameters: String,
    pub context: String,
}

impl ApprovalRequest {
    pub fn sanitize_context(context: impl Into<String>) -> String {
        let context: String = context.into();
        if context.is_empty() {
            return context;
        }

        let context = context.trim();
        let lines: Vec<&str> = context.lines().map(str::trim).collect();
        lines.join("\n")
    }
}

impl From<protos::GetApprovalRequest> for ApprovalRequest {
    fn from(value: protos::GetApprovalRequest) -> Self {
        Self {
            id: ApprovalID::new(),
            name: value.name,
            parameters: value.parameters,
            context: Self::sanitize_context(value.context),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: ApprovalID,
    pub approved: bool,
    pub parameters: String,
    pub approver: Approver,
}

impl From<ApprovalResponse> for protos::GetApprovalResponse {
    fn from(value: ApprovalResponse) -> Self {
        let approved = value.approved;
        let parameters = value.parameters;
        Self { approved, parameters }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approver {
    pub name: String,
    pub email: String,
}
