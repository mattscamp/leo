use leo::output::Output;
use leo::filter::Filter;

use std::path::Path;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use super::Search;

pub struct SimpleSearch {}

impl SimpleSearch {
    pub fn new() -> SimpleSearch {
        SimpleSearch {}
    }
}

impl Search for SimpleSearch {
    fn find(&self, output: Box<Output + Send>, query: String, path: String) {
        let walk = WalkDir::new(&path).into_iter();

        for entry in walk.filter_entry(|e| !e.is_hidden()) {
            match entry {
                Ok(entry) => super::process_entry(&entry, &entry.path(), &query, &output),
                _ => print!("Error with entry"),
            }
        }
    }
}
