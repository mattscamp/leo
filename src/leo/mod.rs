mod search;
mod output;
mod filter;

use self::search::Search;
use self::output::Output;

use std::thread;
use std::path::PathBuf;
use regex::Regex;

use matcha::Match;

const DEFAULT_MAX_THREAD: u32 = 2;

pub struct Leo {
    pub query: Option<String>,
    pub path: Option<String>,
    pub output_strategy: Option<String>,
    pub max_threads: u32,
    pub options: Filters,
}

pub struct Filters {
    recursive: bool,
    regex: bool,
    case_sensitive: bool,
    follow_symlink: bool,
    hidden_files: bool,
    binary_files: bool,
    ignore_files: bool,
}

#[derive(Debug)]
pub struct Matches<'a> {
    path: PathBuf,
    results: Vec<Match>,
    buffer: &'a [u8],
}

impl Leo {
    pub fn new() -> Leo {
        Leo {
            query: None,
            path: None,
            output_strategy: None,
            max_threads: DEFAULT_MAX_THREAD,
            options: Filters {
                recursive: true,
                regex: false,
                case_sensitive: false,
                follow_symlink: false,
                hidden_files: false,
                binary_files: false,
                ignore_files: false,
            },
        }
    }
}

pub fn execute(leo: Leo) {
    let query = match leo.query {
        Some(q) => q,
        None => panic!("No query"),
    };

    let path = match leo.path {
        Some(p) => p.to_string(),
        None => "./".to_string(),
    };

    let output = Output::new();

    Search::start(output, query, path, &leo.options);
}
