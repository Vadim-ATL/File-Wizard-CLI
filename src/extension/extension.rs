use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};


pub fn extension(entries: fs::ReadDir, dry_run: bool, path: &Path) -> Result<(), anyhow::Error> {

    let mut extension_map: HashMap<String, Vec<String>> = HashMap::new();

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
    
    for (ext, files) in extension_map{
        println!("Extension: {}", ext);

        let dir_name = path.join(&ext);
        
        if !dry_run {
            fs::create_dir_all(&dir_name)
                .with_context(|| format!("Failed to create directory for {}", ext))?;
        }

        for file in files {
            let source = path.join(&file);
            let target = dir_name.join(&file);

            println!("  Moving  {} to {}", file, target.display());

            if !dry_run {
                fs::rename(&source, &target)
                .with_context(|| format!("Failed to move file {}", file));
            }
        }
    }
    
    Ok(())
}