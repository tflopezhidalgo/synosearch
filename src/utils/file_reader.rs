use std::fs::File;
use std::sync::Arc;
use std::io::{BufReader, BufRead, Error};

use crate::Logger;

const MESSAGE_INIT: &str = "Read file with words";
const MESSAGE_SPLIT: &str = "Split file into vector";
const MESSAGE_RETURN: &str = "Return vectors of words";

/// FileReader struct
pub struct FileReader {
    filename: String,
    logger: Arc<Logger>,
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
        self.logger.info(MESSAGE_INIT.to_string());

        let file = File::open(&self.filename)?;
        let reader = BufReader::new(file);

        let mut words = vec![];
        for line in reader.lines() {
            words.push(line?.to_string());
        }

        self.logger.info(MESSAGE_SPLIT.to_string());
        words.retain(|x| (x != "" && x != " "));

        self.logger.info(MESSAGE_RETURN.to_string());
        self.logger.info(format!("Lista palabras: {:?}", words));

        Ok(words)
    }
}
