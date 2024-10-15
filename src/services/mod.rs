pub mod receiver;

// Import common modules below.
use crate::protos;
use crate::types::*;
use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

pub struct Phantasm {
    approvals: Mutex<VecDeque<ApprovalTask>>,
}

impl Phantasm {
    pub fn open() -> Result<Self, Box<dyn Error>> {
        Self::setup_dir()?;
        Ok(Phantasm { approvals: Mutex::new(VecDeque::new()) })
    }

    fn insert(&self, task: &ApprovalTask) -> Result<(), Status> {
        let mut approvals = self.approvals.lock().unwrap();
        approvals.push_back(task.to_owned());
        Ok(())
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
