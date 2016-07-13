extern crate rustc_serialize;
extern crate walkdir;
extern crate ansi_term;
extern crate spmc;
extern crate getopts;

mod leo;

use std::env;
use leo::Leo;
use getopts::{Options, Matches};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn help(opts: Options) -> () {
  let brief = format!("Usage: leo <query> --options");
    print!("{}", opts.usage(&brief));
}

fn version() -> () {
  print!("leo {}", VERSION);
}

fn parse_args(args: Vec<String>, l: &mut Leo) {
  let mut opts = Options::new();
    opts.optopt("s", "strategy", "define search strategy", "parallel|basic"); // @todo
    opts.optopt("p", "path", "search the defined directory recursively", "/tmp/leo"); // @todo
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print current version");
  
  let matches = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(f) => { panic!(f.to_string()) }
  };
  
  if matches.opt_present("h") {
    help(opts);
    std::process::exit(1);
  }

  if matches.opt_present("v") {
    version();
    std::process::exit(1);
  }

  l.query = Some(args[1].clone());
  
  parse_opts(matches, l);
}

fn parse_opts(matches: Matches, l: &mut Leo) {
  // -p || --path
  // Override default path (current dir)
  match matches.opt_str("p") {
    Some(p) => {
      l.path = Some(p)
    },
    None => {
      l.path = Some("./".to_string())
    }
  }
  // -s || --strategy
  // Override default search strategy
  match matches.opt_str("s") {
    Some(s) => {
      l.search_strategy = s.to_string();
    },
    None => {}
  }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut l = Leo::new();

    match args.len() {
      // no query or path
      1 => {
        println!("My name is Leo. Try passing some arguments!");
        return;
      },
      _ => {
        parse_args(args, &mut l);       
      }
    }
    
    leo::execute(l);
}
