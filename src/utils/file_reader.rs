use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::sync::Arc;

use crate::Logger;

/// FileReader struct
pub struct FileReader {
    filename: String,
    logger: Arc<Logger>,
}

impl Display for FileReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileReader")
    }
}

impl FileReader {
    pub fn new(filename: String, logger: Arc<Logger>) -> Self {
        println!("Taking words from {:?}", filename);
        FileReader { filename, logger }
    }

    /// Returns a result that can be a vector of words from
    /// `self.filename` splitting by newline separator or a
    /// io::Error.
    pub fn get_words(&self) -> Result<Vec<String>, Error> {
        let file = File::open(&self.filename)?;
        self.logger
            .info(format!("[{}] Opened file: {:?}", self, self.filename));
        let reader = BufReader::new(file);

        let mut words = vec![];
        for line in reader.lines() {
            words.push(line?.to_string());
        }

        words.retain(|x| (x != "" && x != " "));

        self.logger
            .info(format!("[{}] Readed word list: {:?}", self, words));

        Ok(words)
    }
}
