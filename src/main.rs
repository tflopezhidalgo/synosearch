mod parsing;
mod logger;

use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};

use crate::logger::Logger;
use std::{thread};

const FILENAME: &str = "src/log.txt";


fn main() {
    let logger = Logger::new(FILENAME);
    let mut threads = Vec::new();
    for _ in 0..6 {
        let logger_clone = logger.clone();

        let thread = thread::spawn(move || {
            logger_clone.write("Mensaje\n".to_string());

        });
        threads.push(thread);
    }

    for thread in threads {
        match thread.join() {
            Ok(m) => m,
            Err(error) => panic!("Problem creating the file: {:?}", error)
        };
    }


    let p1 = &ThesaurusProvider {url: "".to_string()};
    let p2 = &YourDictionaryProvider {url: "".to_string()};
    let p3 = &MarianWebsterProvider {url: "".to_string()};

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for p in providers {
        println!("{:?}", p.parse("car".to_string()));
    }
}
