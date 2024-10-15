use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskID(Uuid);

impl TaskID {
    /// Create a new random UUID version 4 as a task ID.
    pub fn new() -> Self {
        TaskID(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct ApprovalTask {
    pub id: TaskID,
    pub name: String,
    pub parameters: String,
}
