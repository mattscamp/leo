mod simple_output;

use std::path::Path;

pub use self::simple_output::SimpleOutput;

pub trait Output: OutputClone {
    fn result(&self, path: &Path, line_n: usize, line_c: String);
}

trait OutputClone {
    fn clone_box(&self) -> Box<Output + Send>;
}

impl<T> OutputClone for T
    where T: 'static + Output + Send + Clone
{
    fn clone_box(&self) -> Box<Output + Send> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Output + Send> {
    fn clone(&self) -> Box<Output + Send> {
        self.clone_box()
    }
}
