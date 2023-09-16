use std::path::PathBuf;

pub fn trim_and_convert(file_path: &PathBuf) -> String {
    let content = std::fs::read_to_string(file_path).unwrap();
    let content = content.trim();
    let content = content.to_lowercase();

    content
}