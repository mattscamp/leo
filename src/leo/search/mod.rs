mod simple_search;
mod parallel_search;

pub use self::simple_search::SimpleSearch;
pub use self::parallel_search::ParallelSearch;

use leo::output::Output;

pub trait Search {
    fn find(&self, output: Box<Output>, query: String, path: String);
}