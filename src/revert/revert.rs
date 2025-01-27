use std::fs;
use anyhow::Error;
use anyhow::{Result, Context};
use crate::types::common::FileMove; 
use crate::types::common::FileMoveTracker; 

pub fn revert_moves(result: &Result<FileMoveTracker, Error>) -> Result<(), Error> {
    match result {
        Ok(tracker) => {
            for file_move in &tracker.moves {
                fs::rename(&file_move.target, &file_move.source)
                    .with_context(|| format!("Failed to revert move: {:?} -> {:?}", file_move.target, file_move.source))?;
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to revert moves: {}", e);
            Err(anyhow::Error::msg(e.to_string()))
        }
    }
}

