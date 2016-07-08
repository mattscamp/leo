mod simple_search;
mod parallel_search;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use leo::output::Output;
use leo::filter::Filter;

pub use self::simple_search::SimpleSearch;
pub use self::parallel_search::ParallelSearch;

fn li<T>((n, x): (usize, T)) -> (usize, T) {
    (n + 1, x)
}

fn process_entry<F: Filter>(e: &F, p: &Path, q: &String, o: &Box<Output + Send>) {
    if e.passes_filters() {
        let mut f = File::open(p);
        let mut reader = BufReader::new(f.unwrap());

        for (line_n, line) in reader.lines().enumerate().map(li) {
            match line {
                Ok(x) => {
                    if (x.contains(q)) {
                        o.result(p, line_n, x);
                    }
                }
                Err(x) => continue,
            }
        }
    }
}

pub trait Search {
    fn find(&self, output: Box<Output + Send>, query: String, path: String);
}
