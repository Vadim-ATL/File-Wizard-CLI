use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use anyhow::{Result, Context};


pub fn file_type(entries: fs::ReadDir, path: &Path) -> Result<(), std::io::Error> {
    
    let path_buf = path.to_path_buf();

    let types: Vec<&str> = vec!["Images", "Music", "Video", "Other"];
      
    for type_name in types {
        let mut path_buf = path.to_path_buf();
        path_buf.push(type_name);
        fs::create_dir_all(&path_buf).expect("Failed to create directory");
    }
    
    let images = vec![
            "svg".to_string(),
            "raw".to_string(),
            "tiff".to_string(),
            "bmp".to_string(),
            "gif".to_string(),
            "webp".to_string(),
            "jpg".to_string(),
            "png".to_string(),
            "jpeg".to_string(),
        ];

    let music = vec![
            "ogg".to_string(),
            "m4a".to_string(),
            "flac".to_string(),
            "aac".to_string(),
            "mp3".to_string(),
            "wav".to_string(),
        ];

    let video = vec![
            "webm".to_string(),
            "wmv".to_string(),
            "flv".to_string(),
            "mov".to_string(),
            "mkv".to_string(),
            "mp4".to_string(),
            "avi".to_string(),
        ];
        
    for entry in entries{

        let entry = entry?;
        let path = entry.path();
        
        if path.is_file(){

            let extension = path.extension()
            .and_then(|e
                | e.to_str())
            .unwrap_or("unknown")
            .to_string();

            let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

            if images.contains(&extension){
                let dest_path: PathBuf = path_buf.join("Images").join(&filename);

                println!("File extension is {}", extension);
                println!("Moving file:");
                println!("From: {}", path.display());
                println!("To:   {}", dest_path.display());
                println!("Filename {}", filename);

                fs::rename(&path, dest_path)
                .with_context(|| format!("Failed to move file {}", filename));
            }

            if music.contains(&extension){
                let dest_path: PathBuf = path_buf.join("Music").join(&filename);

                println!("File extension is {}", extension);
                println!("Moving file:");
                println!("From: {}", path.display());
                println!("To:   {}", dest_path.display());

                fs::rename(&path, dest_path)
                .with_context(|| format!("Failed to move file {}", filename));

            }
        }
    }
    Ok(())
    
}