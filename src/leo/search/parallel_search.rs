use leo::output::Output;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use super::Search;

pub struct ParallelSearch {}

const DEFAULT_THREADS: u32 = 1;

impl ParallelSearch {
	pub fn new() -> ParallelSearch {
		ParallelSearch {}
	}
}

impl Search for ParallelSearch {
    fn find(&self, output: Box<Output>, query: String, path: String) {
    }
}