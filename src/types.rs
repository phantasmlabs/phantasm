use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionID(Uuid);

impl ConnectionID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
