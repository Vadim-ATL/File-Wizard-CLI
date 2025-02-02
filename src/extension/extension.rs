use std::collections::HashMap;
use std::{fs,fs::File};
use anyhow::{Result, Context};
use log::{info};
use simplelog::*;
use chrono::Utc;
use std::path::Path;

use crate::types::common::FileMove; 
use crate::types::common::FileMoveTracker; 

pub fn extension(entries: fs::ReadDir, dry_run: bool, path: &Path) -> Result<FileMoveTracker, anyhow::Error> {
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%SZ").to_string();
    let log_dir = Path::new("./log");
    
    let log_filename = log_dir.join(format!("log_{}.txt", timestamp));
    
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(),File::create(&log_filename).unwrap()),
        ]
    ).unwrap();


    let mut extension_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut moved_files: Vec<FileMove> = Vec::new();

    for entry in entries{
        let entry = entry?;
        let path = entry.path();
    
        if path.is_file(){
            let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknown")
            .to_string();
    
            let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
    
            extension_map.entry(extension)
                .or_default()
                .push(filename);
        }
    }
    info!("Launch the File Wizard");

    for (ext, files) in extension_map{
        println!("Extension: {}", ext);

        let dir_name: std::path::PathBuf = path.join(&ext);
        
        if !dry_run {
            fs::create_dir_all(&dir_name)
                .with_context(|| format!("Failed to create directory for {}", ext))?;
        }

        for file in files {
            let source = path.join(&file);
            let target = dir_name.join(&file);

            println!("  Moving  {} to {}", file, target.display());
            info!("  Moving  {} to {}", file, target.display());

            if !dry_run {
                fs::rename(&source, &target)
                .with_context(|| format!("Failed to move file {}", file))?;
            }
            println!("Pushing {} and {}", path.display(), dir_name.display());
            moved_files.push(FileMove {
                source: path.to_str().unwrap_or("").to_string(),
                target: dir_name.to_str().unwrap_or("").to_string(),
            });
        }
    }
    info!("Finished moving");
    Ok(FileMoveTracker { moves: moved_files })

}