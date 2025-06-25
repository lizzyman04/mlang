use std::fs;
use std::path::Path;

pub fn load_markdown_file(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path));
    }
    fs::read_to_string(path)
        .map_err(|e| format!("Error reading file {}: {}", file_path, e))
}