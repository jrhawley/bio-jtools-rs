//! Methods for filtering HTS records.

use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

pub trait RecordFilter {
    /// Output file name (or STDOUT if file not given)
    fn output(&self) -> Option<&PathBuf>;

    /// Return what type of writer to use here (STDOUT, a file, or something else)
    fn writer_output(&self) -> Result<Box<dyn Write>, io::Error> {
        match self.output() {
            Some(path) => File::open(path).map(|f| Box::new(f) as Box<dyn Write>),
            None => Ok(Box::new(io::stdout())),
        }
    }
}
