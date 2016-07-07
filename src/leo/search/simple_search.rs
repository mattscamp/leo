use leo::output::Output;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use super::Search;

pub struct SimpleSearch {}

impl SimpleSearch {
	pub fn new() -> SimpleSearch {
		SimpleSearch {}
	}
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn li<T>((n, x): (usize, T)) -> (usize, T) {
    (n+1, x)
}

// @TODO: Move to a more generic filters module
fn is_supported_type(path: &Path) -> bool {
	return path.is_file()
}

impl Search for SimpleSearch {
    fn find(&self, output: Box<Output>, query: String, path: String) {
   		let walk = WalkDir::new(&path).into_iter();

		for entry in walk.filter_entry(|e| !is_hidden(e)) {
		    let entry = entry.unwrap();
		    let path = entry.path();

			if is_supported_type(&path) {
				let mut f = File::open(&path);
				let mut reader = BufReader::new(f.unwrap());

				for (line_n, line) in reader.lines().enumerate().map(li) {
					match line {
			            Ok(x) => {
			            	if(x.contains(&query)) {
			            		output.result(path, line_n, x);
			            	}
			            },
			            Err(x) => continue,
			        }
				}
			}
		}
    }
}