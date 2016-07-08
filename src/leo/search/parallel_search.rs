use leo::output::Output;
use leo::filter::Filter;

use walkdir::{DirEntry, WalkDir, WalkDirIterator};

use std::thread;
use std::path::Path;
use std::time::Duration;

use spmc::*;

use super::Search;

pub struct ParallelSearch {}

const DEFAULT_THREADS: u32 = 3;

impl ParallelSearch {
	pub fn new() -> ParallelSearch {
		ParallelSearch {}
	}
}

impl Search for ParallelSearch {
    fn find(&self, output: Box<Output + Send>, query: String, path: String) {
   		let walk = WalkDir::new(&path).into_iter();
	    let (tx, rx) = channel::<Option<DirEntry>>();
		let mut handles = Vec::new();

		// Spawn some worker threads to process files
		for n in 0..DEFAULT_THREADS {
		    let rx = rx.clone();
		    let q = query.clone();
		    let o = output.clone();
		    handles.push(thread::spawn(move || {
		    	loop {
		    		let msg = rx.recv().unwrap();
		    		match msg {
		                Some(e) => {
		                	super::process_entry(&e, &e.path(), &q, &o);
		                },
		                None => {
		                	break;
		                }
		            }
		        }
		    }));
		}

		// Our parent thread walks dirs and sends files to children
   		for entry in walk.filter_entry(|e| !e.is_hidden()) {
			match entry {
				Ok(entry) => { 
					tx.send(Some(entry)).unwrap();
				},
				_ => print!("Unreadable file")
			}
		}

		for n in 0..DEFAULT_THREADS {
			tx.send(None);
		}

		for handle in handles {
			handle.join().unwrap();
		}
    }
}