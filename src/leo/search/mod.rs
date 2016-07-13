mod simple_search;
mod parallel_search;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::str;

use leo::output::Output;
use leo::filter::Filter;

pub use self::simple_search::SimpleSearch;
pub use self::parallel_search::ParallelSearch;

fn li<T>((n, x): (usize, T)) -> (usize, T) {
    (n + 1, x)
}

fn process_entry<F: Filter>(e: &F, p: &Path, q: &String, o: &Box<Output + Send>) {
    if e.passes_filters() {
        let mut f = File::open(p).unwrap();;
        let len = f.metadata().unwrap().len();
        let mut buffer = String::with_capacity(len as usize);

        f.read_to_string(&mut buffer);
        // We dont want to loop through the file if there is nothing to find
        if buffer.contains(q) {
            for (line_n, line) in buffer.lines().enumerate().map(li) {
                if line.contains(q) {
                    o.result(p, line_n, line);
                }
            }
        }
    }
}

pub trait Search {
    fn find(&self, output: Box<Output + Send>, query: String, path: String);
}
