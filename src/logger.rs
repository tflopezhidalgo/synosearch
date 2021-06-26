use std::io::Write;
use std::fs::File;
use std::sync::{Arc, Mutex};

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