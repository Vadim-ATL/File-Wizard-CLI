use std::{fs,fs::File};
use anyhow::Error;
use anyhow::{Result, Context};
use log::info;
use simplelog::*;
use crate::types::common::FileMove; 
use crate::types::common::FileMoveTracker; 
use chrono::Utc;
use std::path::Path;


pub fn revert_moves(result: &Result<FileMoveTracker, Error>) -> Result<(), Error> {
    
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let log_dir = Path::new("./log");
    
    let log_filename = log_dir.join(format!("log_{}.txt", timestamp));
    
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(),File::create(&log_filename).unwrap()),
        ]
    ).unwrap();
    
    match result {
        Ok(tracker) => {
            for file_move in &tracker.moves {
                println!("  Reverting - Moving files  {} back to {}", file_move.target, file_move.source);
                info!("  Reverting - Moving files  {} back to {}", file_move.target, file_move.source);
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

