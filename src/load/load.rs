use serde_json;
use std::fs::File;
use chrono::Utc;
use std::{fs::{self}, path::Path};
use anyhow::{Result, Context};
use std::cmp::Ordering;


use crate::types::common::FileMove; 
use crate::types::common::FileMoveTracker; 

pub fn save_moved_files(tracker: &FileMoveTracker) -> Result<(), anyhow::Error> {

    let moved_dir = Path::new("./moved_files");
    
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let moved_files = moved_dir.join(format!("moved_files_{}.txt", timestamp));

    let file = File::create(&moved_files)?;
    serde_json::to_writer(file, tracker)?;
    Ok(())
}


pub fn load_moved_file() -> Result<FileMoveTracker, anyhow::Error> {
    let moved_files_dir = Path::new("./moved_files");

    let files = fs::read_dir(moved_files_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect::<Vec<_>>();

    if files.is_empty() {
        return Err(anyhow::anyhow!("No moved files found").into());
    }

    let latest_file = files
        .iter()
        .max_by(|a, b| {
            let a_path = a.path();
            let b_path = b.path();

            let a_timestamp = a_path.file_name().and_then(|f| f.to_str());
            let b_timestamp = b_path.file_name().and_then(|f| f.to_str());

            match (a_timestamp, b_timestamp) {
                (Some(a), Some(b)) => a.cmp(b),
                _ => Ordering::Equal,
            }
        })
        .context("Failed to determine the latest file")?;
    let file_path = latest_file.path();

    let file = File::open(&file_path).with_context(|| format!("Failed to open file: {}", file_path.display()))?;
    let tracker: FileMoveTracker = serde_json::from_reader(file)
        .with_context(|| format!("Failed to deserialize file: {}", file_path.display()))?;

    Ok(tracker)
}
