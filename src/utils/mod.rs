use std::collections::HashMap;
use std::io::Write;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::fs;


/* COUNTER */

pub struct Counter;

impl Counter {
    pub fn count(synonimous: Vec<String>) { 
        let mut synonimous_counter = HashMap::new();
        for s in synonimous {
            let entry = synonimous_counter.entry(s).or_insert(0);
            *entry += 1;
        }

        /*
        for c in synonimous_counter {
            println!("{:?}", c);
        }
        */
        println!("{:?}", synonimous_counter);
    }
}

/* LOGGER */ 

#[derive(Clone, Debug)]
pub struct Logger {
    file: Arc<Mutex<std::fs::File>>
}

impl Logger {
    pub fn new(filename: &str) -> Self {
        let f = match File::create(filename) {
            Ok(file) => file,
            Err(error) => panic!("Problem creating the file: {:?}", error)
        };
        let file = Arc::new(Mutex::new(f));
        return Logger{file};
    }

    pub fn write(&self, message: String) {
        let mut file = match self.file.lock() {
            Ok(m) => m,
            Err(error) => panic!("Problem lock file: {:?}", error)
        };
        match file.write_all(message.as_bytes()) {
            Ok(m) => m,
            Err(error) => panic!("Problem writting the file: {:?}", error)
        };
    }
}

/* FILEREADER */

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