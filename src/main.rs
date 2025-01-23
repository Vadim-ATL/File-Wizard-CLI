use clap::Parser;
use std::{collections::hash_map::Entry, path::Path};
 
use std::fs;

use std::collections::HashMap;
use anyhow::{Result, Context};

#[derive(Parser)]
#[command(author = "Vadim Atlassov", version = "1.0.0", about = "The CLI file wizard for smart file organization", long_about = "A Powerful CLI file wizard for smart file organization, search, and manipulation.")]
struct Args{
    #[arg(short, long, default_value = ".")]
    path: String,
    #[arg(short, long)]
    dry_run: bool,
}


fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    let mut extension_map: HashMap<String, Vec<String>> = HashMap::new();

    let entries = fs::read_dir(path)
    .with_context(|| format!("Failed to read directory {}", args.path))?;

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
        
        if !args.dry_run {
            fs::create_dir_all(&dir_name)
                .with_context(|| format!("Failed to create directory for {}", ext))?;
        }

        for file in files {
            let source = path.join(&file);
            let target = dir_name.join(&file);

            println!("  Moving  {} to {}", file, target.display());

            if !args.dry_run {
                fs::rename(&source, &target)
                .with_context(|| format!("Failed to move file {}", file));
            }
        }
    }
    //println!("Working in direction {}", args.path);
    Ok(())
}
