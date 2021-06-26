mod parsing;
mod logger;
mod counter;

use crate::counter::Counter;
use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarrianWebsterProvider, 
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
            Err(error) => panic!("Problem in join threads: {:?}", error)
        };
    }


    let p1 = &ThesaurusProvider;
    let p2 = &YourDictionaryProvider;
    let p3 = &MarrianWebsterProvider;

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];
    let mut synonimous = Vec::new();

    for p in providers {
        synonimous.append(&mut p.parse("car".to_string()));
    }

    Counter::count(synonimous);
}
