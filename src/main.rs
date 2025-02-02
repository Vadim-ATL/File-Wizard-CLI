mod file_type;
mod extension;
mod load;
mod types;
mod revert;
mod utils;

use utils::setup_dirs::setup_dir;
use clap::Parser;
use std::path::Path;
use std::fs;

use file_type::file_type::file_type;
use extension::extension::extension;
use load::load::load_moved_file;
use load::load::save_moved_files;
use revert::revert::revert_moves;

use anyhow::{Result, Context};


#[derive(Parser)]
#[command(author = "Vadim Atlassov", version = "1.0.0", about = "The CLI file wizard for smart file organization", long_about = "A Powerful CLI file wizard for smart file organization, search, and manipulation.", before_help("\n \n ✨ Welcome to the CLI File Wizard ✨"))]
struct Args{
    ///Path to organise files 
    #[arg(short, long, default_value = "/Users/vadimatlassov/Downloads/files")]
    path: String,
    ///Test run for showing how files will be organized
    #[arg(short, long)]
    dry_run: bool,
    ///Organize files by types
    #[arg(short, long)]
    types: bool,
    ///Revert changes
    #[arg(short, long)]
    revert: bool

}


fn main() -> Result<()> {

    let args = Args::parse();
    let path = Path::new(&args.path);
    
    println!("Working in direction {} \n", args.path);
    
    let entries = fs::read_dir(path)
    .with_context(|| format!("Failed to read directory {}", args.path))?;


    setup_dir();

    if args.types {
        println!("Invoking organization by file_type...\n");
        file_type(entries,args.dry_run, path);
    } else {
        println!("Invoking organization by extension... \n");
        let tracked_files = extension(entries, args.dry_run, path)?;
        save_moved_files(&tracked_files)?;
    }

    if args.revert {
        println!("Reverting changes by extension...\n");
        let loaded_files = load_moved_file();
        revert_moves(&loaded_files);
    }
    
    Ok(())
}

