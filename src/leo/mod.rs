mod search;
mod output;
mod filter;

use self::search::{Search, SimpleSearch, ParallelSearch};
use self::output::{Output, SimpleOutput};

pub struct Leo {
  pub query: Option<String>,
  pub path: Option<String>,
  pub search_strategy: String
}

impl Leo {
    pub fn new() -> Leo {
        Leo {
          query: None,
          path: None,
          search_strategy: "parallel".to_string(),
        }
    }
}

fn get_search_strategy(strategy: String) -> Box<Search> {
    match strategy.as_str() {
        "basic" => Box::new(SimpleSearch::new()) as Box<Search>,
        _ => Box::new(ParallelSearch::new()) as Box<Search>
    }
}

fn get_output_strategy(strategy: &str) -> Box<Output + Send> {
    match strategy {
        _ => Box::new(SimpleOutput::new()) as Box<Output + Send>
    }
}

pub fn execute(leo: Leo) {
  let query = leo.query.expect("No query provided.");
	let path = leo.path.expect("No path provided.");
	let search_strategy = get_search_strategy(leo.search_strategy);
	let output_strategy = get_output_strategy("");

  search_strategy.find(output_strategy, query, path);
}
