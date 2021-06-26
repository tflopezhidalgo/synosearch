use crate::Arc;
use std::fs;
use crate::Logger;

const SPLIT_CHAR: &str = "\n";
const MESSAGE_INIT: &str = "INFO: Read file with words\n";
const MESSAGE_SPLIT: &str = "INFO: Split file lines into vector\n";
const MESSAGE_RETURN: &str = "INFO: Return vectors of words\n";

pub struct FileReader {
    filename: String, 
    logger: Arc<Logger>
}

impl FileReader {
    pub fn new(filename: String, logger: Arc<Logger>) -> Self {
        FileReader{filename, logger}
    }
    pub fn get_words(&self) -> Vec<String> {
        self.logger.write(MESSAGE_INIT.to_string());

        let contents = fs::read_to_string(&self.filename)
            .expect("Something went wrong reading the file"); 

        self.logger.write(MESSAGE_SPLIT.to_string());
        let words = contents.split(SPLIT_CHAR).collect::<Vec<&str>>();
        let mut vec = Vec::new();

        for w in words.into_iter() {
            vec.push(w.to_string());
        }
        self.logger.write(MESSAGE_RETURN.to_string());
        return vec;
    }   
}