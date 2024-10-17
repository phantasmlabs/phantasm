use crate::protos;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: ApprovalID,
    pub name: String,
    pub parameters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResponse {
    pub id: ApprovalID,
    pub status: ApprovalStatus,
    pub parameters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Approved,
    Modified,
    Denied,
}

impl From<ApprovalStatus> for protos::ApprovalStatus {
    fn from(status: ApprovalStatus) -> Self {
        match status {
            ApprovalStatus::Approved => Self::Approved,
            ApprovalStatus::Modified => Self::Modified,
            ApprovalStatus::Denied => Self::Denied,
        }
    }
}
