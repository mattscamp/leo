use ansi_term::Style;
use ansi_term::Colour::Blue;

use std::path::Path;
use std::str;

use leo::{Leo, Matches, Filters};

use matcha::Match;

pub struct Output {}

impl Output {
	pub fn new() -> Output {
		Output {}
	}

    pub fn print(&self, matches: Matches) {
    	// Print out the current file these matches are in
        match matches.path.to_str() {
        	Some(p) => println!("{}", Blue.bold().paint(p)),
        	None => panic!("No path")
        }

        let mut line_n = 0;
        let mut pos = 0;
        let mut last_pos = 0;

        for m in matches.results {
        	// Figure out which line the current match is on
            while pos < m.start {
                if matches.buffer[pos] == b'\n' {
                    line_n += 1;
                    last_pos = pos;
                }
                pos += 1;
            }

            let mut end = m.end;
            // Find the end of the match line for printing context
            while matches.buffer.len() > end {
	            if matches.buffer[end] == b'\n' { end -= 1; break; }
	            end += 1;
	        }

	        // The line number
	        let line = format!("{}", line_n);

	        // The part of the line before our match
	        let line_b = match str::from_utf8(&matches.buffer[(last_pos + 1)..m.start]) {
	        	Ok(s) => s.trim(),
	        	_ => ""
	        };
	        // Our actual match
	        let line_m = match str::from_utf8(&matches.buffer[m.start..m.end]) {
	        	Ok(s) => Style::new().underline().paint(s),
	        	_ => ""
	        };
	        // The part of the line after our match
	        let line_e = match str::from_utf8(&matches.buffer[m.end..end]) {
	        	Ok(s) => s.trim(),
	        	_ => ""
	        };

            println!("{}: {}{}{}\n", Style::new().bold().paint(line), line_b, line_m, line_e);
        }
    }
}
