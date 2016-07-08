use std::path::Path;
use walkdir::{DirEntry, Error};

pub trait Filter {
    fn passes_filters(&self) -> bool;
    fn is_hidden(&self) -> bool;
    fn is_supported_type(&self) -> bool;
}
// Move to own file
impl Filter for DirEntry {
    fn passes_filters(&self) -> bool {
        !self.is_hidden() && self.is_supported_type()
    }

    fn is_hidden(&self) -> bool {
        self.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    fn is_supported_type(&self) -> bool {
        self.file_type().is_file()
    }
}
