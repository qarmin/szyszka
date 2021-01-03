pub struct ResultEntries {
    pub entries: Vec<FileEntry>,
}

#[derive(Clone)]
pub struct FileEntry {
    pub current_name: String,
    pub future_names: Vec<String>,
    pub path: String,
    pub size: u64,
    pub modification_date: u64,
    pub creation_date: u64,
    pub opening_date: u64,
    pub image_dimensions: u64,
}

impl FileEntry {}
