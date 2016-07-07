extern crate walkdir;
extern crate ansi_term;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

use ansi_term::Style;
use ansi_term::Colour::{Blue};
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

pub struct Leo {
	query: String,
	path: String,
}

// These will be used later on
const TYPES: &'static [ &'static str ] = &[ 
	"asm", "as", 
	"bat", "cmd", 
	"c", "h", "xs",
	"cpp", "cc", "C", "cxx", "m", "hpp", "hh", "h", "H", "hxx",
	"go",
	"htm", "html", "shtml", "xhtml",
	"css",
	"js", "jsx",
	"rb", "rhtml", "rjs", "rxml", "erb", "rake", "spec",
	"jsp", "jspx", "jhtm", "jhtml",
	"java", "properties",
	"xml", "dtd", "xsl", "xslt", "ent",
	"markdown", "mdown", "mdwn", "mkdn", "mkd", "md",
	"ml", "mli", "mll", "mly",
	"sh", "bash", "csh", "tcsh", "ksh", "zsh", "fish",
	"yaml", "yml",
	"json", 
	"toml", 
	"rs"
];

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn li<T>((n, x): (usize, T)) -> (usize, T) {
    (n+1, x)
}

// @TODO: Move to a more generic filters module
fn is_supported_type(path: &Path) -> bool {
	return path.is_file()
}

impl Leo {
	// Return a new Leo struct
	pub fn new(query: Option<String>, path: Option<String>) -> Leo {
		let query = query.unwrap();
		let path = path.unwrap();
		
		Leo {
			query: query,
			path: path
		}
	}
	// Executes our search, this will grow and be thrown into a more productive file structure
	// but keeping it simple for now
	pub fn search(&mut self) {
		let walk = WalkDir::new(&self.path).into_iter();

		for entry in walk.filter_entry(|e| !is_hidden(e)) {
		    let entry = entry.unwrap();
		    let path = entry.path();

			if is_supported_type(&path) {
				let mut f = File::open(&path);
				let mut reader = BufReader::new(f.unwrap());

				for (line_n, line) in reader.lines().enumerate().map(li) {
					match line {
			            Ok(x) => {
			            	if(x.contains(&self.query)) {
			            		println!("{}:{}\n{}", Blue.bold().paint(path.to_str().unwrap()), Style::new().bold().paint(line_n.to_string()), x.trim());
			            	}
			            },
			            Err(x) => continue,
			        }
				}
			}
		}
	}
}