use std::path::Path;
use walkdir::{DirEntry, Error};

const NULL_BYTE: u8 = b'\0';

pub trait BufFilter {
    fn is_binary(&self) -> bool;
}

pub trait DirFilter {
    fn passes_filters(&self) -> bool;
    fn is_hidden(&self) -> bool;
    fn is_file(&self) -> bool;
}

// Move to own file
impl DirFilter for DirEntry {
    fn passes_filters(&self) -> bool {
        !self.is_hidden() && self.is_file()
    }

    fn is_hidden(&self) -> bool {
        self.file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    fn is_file(&self) -> bool {
        self.file_type().is_file()
    }
}

impl<'a> BufFilter for &'a [u8] {
    fn is_binary(&self) -> bool {
        let len = self.len();
        let total = match len > 256 {
            true => 256,
            false => len,
        };

        for i in 0..total {
            if self[i] == NULL_BYTE {
                return true;
            }
        }

        false
    }
}
