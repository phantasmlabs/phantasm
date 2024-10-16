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
pub struct ApprovalID(Uuid);

impl ApprovalID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
