use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use sys_info::*;

use memmap::{Mmap, Protection};

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use leo::{Leo, Matches, Filters};
use leo::filter::{DirFilter, BufFilter};
use leo::output::Output;

use matcha::Match;
use matcha::tbm::TBMMatcher;
use matcha::simple::SimpleMatcher;

use regex::Regex;

pub struct Search {}

fn matches(q: &String, buf: &[u8], len: usize) -> Vec<Match> {
    match len >= 1000 {
        true => TBMMatcher::matches(q.as_bytes(), buf),
        false => SimpleMatcher::matches(q.as_bytes(), buf),
    }
}

impl Search {
    pub fn start(out: Output, q: String, p: String, options: &Filters) {
        let walk = WalkDir::new(&p).into_iter();
        let mem = mem_info().unwrap();
        for entry in walk.filter_entry(|e| !e.is_hidden()) {
            match entry {
                Ok(entry) => {
                    if entry.passes_filters() {
                        let path = entry.path();
                        let meta = path.metadata().unwrap();
                        let len = meta.len() as usize;

                        if len == 0 {
                            continue; // Dont waste io if not
                        }

                        let mmap = Mmap::open_path(path, Protection::Read).unwrap();
                        let buffer = unsafe { mmap.as_slice() };

                        if !buffer.is_binary() {
                            let matches = matches(&q, &buffer, len);

                            if matches.len() > 0 {
                                let mut res = Matches {
                                    path: entry.path().to_path_buf(),
                                    results: matches,
                                    buffer: buffer,
                                };

                                out.print(res);
                            }
                        }
                    }
                }
                _ => print!("Error with entry"),
            }
        }
    }
}
