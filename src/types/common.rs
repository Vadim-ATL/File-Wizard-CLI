use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMoveTracker {
    pub moves: Vec<FileMove>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMove {
    pub source: String,
    pub target: String,
}
