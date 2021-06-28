use crate::Arc;
use crate::Logger;
use std::fs;

const SPLIT_CHAR: &str = "\n";
const MESSAGE_INIT: &str = "Read file with words";
const MESSAGE_SPLIT: &str = "Split file into vector";
const MESSAGE_RETURN: &str = "Return vectors of words";

pub struct FileReader {
    filename: String,
    logger: Arc<Logger>,
}

impl FileReader {
    pub fn new(filename: String, logger: Arc<Logger>) -> Self {
        FileReader { filename, logger }
    }
    pub fn get_words(&self) -> Vec<String> {
        self.logger.info(MESSAGE_INIT.to_string());

        let contents =
            fs::read_to_string(&self.filename).expect("Something went wrong reading the file");

        self.logger.info(MESSAGE_SPLIT.to_string());
        let words = contents.split(SPLIT_CHAR).collect::<Vec<&str>>();
        let mut vec = Vec::new();

        for w in words.into_iter() {
            vec.push(w.to_string());
        }
        vec.retain(|x| x != "");
        vec.retain(|x| x != " ");
        self.logger.info(MESSAGE_RETURN.to_string());
        self.logger.info(format!("Lista palabras: {:?}", vec));
        return vec;
    }
}
