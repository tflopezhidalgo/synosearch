use std::fs;

const SPLIT_CHAR: &str = "\n";


pub struct FileReader {
    filename: String
}

impl FileReader {
    pub fn new(filename: String) -> Self {
        FileReader{filename}
    }
    pub fn get_words(&self) -> Vec<String> {
        let contents = fs::read_to_string(&self.filename)
            .expect("Something went wrong reading the file"); 

        let words = contents.split(SPLIT_CHAR).collect::<Vec<&str>>();
        let mut vec = Vec::new();
        for w in words.into_iter() {
            vec.push(w.to_string());
        }
        return vec;
    }   
}