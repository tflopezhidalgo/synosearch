use crate::Arc;
use crate::Logger;
use std::io;
use std::fs;

const SPLIT_CHAR: &str = "\n";
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
        FileReader { filename, logger }
    }

    /// Returns a result that can be a vector of words from 
    /// `self.filename` splitting by newline separator or a
    /// io::Error.
    pub fn get_words(&self) -> Result<Vec<String>, io::Error> {
        self.logger.info(MESSAGE_INIT.to_string());

        let contents = match fs::read_to_string(&self.filename) {
            Ok(content) => content,
            Err(e) => return Err(e),
        };

        self.logger.info(MESSAGE_SPLIT.to_string());
        let mut words = contents
            .split(SPLIT_CHAR)
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        words.retain(|x| (x != "" && x != " "));

        self.logger.info(MESSAGE_RETURN.to_string());
        self.logger.info(format!("Lista palabras: {:?}", words));

        Ok(words)
    }
}
