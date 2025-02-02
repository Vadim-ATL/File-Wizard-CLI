use std::path::Path;
use std::fs::create_dir_all;

pub fn setup_dir(){
    let dirs = ["./log", "./moved_files"];
    for dir in dirs.iter() {
        let path = Path::new(dir);
        if !path.exists() {
            if let Err(err) = create_dir_all(path) {
                eprintln!("Failed to create directory {}: {}", dir, err);
            }
        }
    }
}
