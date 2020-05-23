use walkdir::DirEntry;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn is_yaml(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(is_filename_yaml)
        .unwrap_or(false)
}

pub fn is_filename_yaml(filename: &str) -> bool {
    filename.ends_with("yml") || filename.ends_with("yaml")
}
