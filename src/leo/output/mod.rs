mod simple_output;

use std::path::Path;

pub use self::simple_output::SimpleOutput;

pub trait Output {
    fn result(&self, path: &Path, line_n: usize, line_c: String);
}