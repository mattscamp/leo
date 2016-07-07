extern crate rustc_serialize;
extern crate docopt;
extern crate leo;

use docopt::Docopt;
use leo::{
    Leo
};

const USAGE: &'static str = "
Leo

Usage:
  leo [<query> [<path>] [options]
  leo --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_query: Option<String>,
    arg_path: Option<String>,
    flag_version: bool,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    
    if (args.flag_version) {
    	return print!("Leo version: {}", VERSION);
    }

    let mut leo = Leo::new(args.arg_query, args.arg_path);
    leo.search();
}