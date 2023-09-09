use crate::text_processing;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub fn scan_folder(path: &str) -> Result<HashMap<PathBuf, String>, Box<dyn Error>> {
    let folder = Path::new(path);
    if !folder.exists() || !folder.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Folder does not exist").into());
    }

    let mut data: HashMap<PathBuf, String> = HashMap::new();
    for entry in fs::read_dir(folder)? {
        let file_path = entry?.path();

        if file_path.is_dir() {
            let nested_data = scan_folder(&file_path.to_str().unwrap());
            for (nested_path, content) in nested_data? {
                data.insert(nested_path, content);
            }
        }

        if file_path.is_file() {
            match file_path.extension() {
                Some(ext) if ext == "txt" || ext == "md" => {
                    let file_contents = fs::read_to_string(&file_path)?;
                    let content_parsed = text_processing::stop_word_removal(&file_contents);
                    data.insert(file_path, content_parsed);
                }
                _ => eprintln!("Sorry, we cannot parse that file type"),
            }
        }
    }
    Ok(data)
}
