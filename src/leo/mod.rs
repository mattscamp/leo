mod search;
mod output;

use self::search::{Search, SimpleSearch, ParallelSearch};
use self::output::{Output, SimpleOutput};

fn get_search_strategy(strategy: &str) -> Box<Search> {
    match strategy {
        "simple" => Box::new(SimpleSearch::new()) as Box<Search>,
        _ => Box::new(ParallelSearch::new()) as Box<Search>
    }
}

fn get_output_strategy(strategy: &str) -> Box<Output> {
    match strategy {
        _ => Box::new(SimpleOutput::new()) as Box<Output>
    }
}

pub fn execute(query: Option<String>, path: Option<String>) {
    let query = query.expect("No query provided.");
	let path = path.expect("No path provided.");
	let search_strategy = get_search_strategy("simple");
	let output_strategy = get_output_strategy("");

    search_strategy.find(output_strategy, query, path);
}
