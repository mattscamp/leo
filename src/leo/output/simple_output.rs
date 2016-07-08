use ansi_term::Style;
use ansi_term::Colour::{Blue}; 
use std::path::Path;

use super::Output;

#[derive(Clone)]
pub struct SimpleOutput {}

impl SimpleOutput {
	pub fn new() -> SimpleOutput {
		SimpleOutput {}
	}
}

impl Output for SimpleOutput {
    fn result(&self, path: &Path, line_n: usize, line_c: String) {
    	let path = path.to_str().expect("No path passed.");
    	let line_n = line_n.to_string();
    	let line_c = line_c.trim();
   		println!("{}:{}\n{}", Blue.bold().paint(path), Style::new().bold().paint(line_n), line_c);
   	}
}