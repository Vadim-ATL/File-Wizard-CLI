mod file_type;
mod extension;

use clap::Parser;
use std::path::Path;
use std::fs;

use file_type::file_type::file_type;
use extension::extension::extension;

use anyhow::{Result, Context};


#[derive(Parser)]
#[command(author = "Vadim Atlassov", version = "1.0.0", about = "The CLI file wizard for smart file organization", long_about = "A Powerful CLI file wizard for smart file organization, search, and manipulation.", before_help("\n \n ✨ Welcome to the CLI File Wizard ✨"))]
struct Args{
    ///Path to organise files 
    #[arg(short, long, default_value = ".")]
    path: String,
    ///Test run for showing how files will be organized
    #[arg(short, long)]
    dry_run: bool,
    ///Organize files by types
    #[arg(short, long)]
    types: bool
}


fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);
    
    println!("Working in direction {}", args.path);

    let entries = fs::read_dir(path)
    .with_context(|| format!("Failed to read directory {}", args.path))?;

    if args.types {
        println!("Invoking organization by file_type...");
        file_type(entries, path);
    } else {
        println!("Invoking organization by extension...");
        extension(entries, args.dry_run, path); 
    }
    
    Ok(())
}
