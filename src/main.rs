mod parsing;
mod logger;
mod counter;
mod file_reader;

use crate::counter::Counter;
use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarrianWebsterProvider, 
    Parser
};

use crate::logger::Logger;
use crate::file_reader::FileReader;
use std::{thread};

use std::process;
use std::env;


const FILENAME: &str = "src/log.txt";

fn try_threads() {
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
}


fn run_parsers(words: Vec<String>) {
    let p1 = &ThesaurusProvider;
    let p2 = &YourDictionaryProvider;
    let p3 = &MarrianWebsterProvider;

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for w in &words {
        let mut synonimous = Vec::new();
        println!("WORD: {}", w);
        for p in &providers {
            synonimous.append(&mut p.parse(w.to_string()));
        }
        Counter::count(synonimous);
    }
}

fn choose_mode(mode:String, filename: String) {
    let words = FileReader::new(filename).get_words();

    if mode.eq("actors") {
        println!("actors");
    } else if mode.eq("threads") {
        println!("threads");
        run_parsers(words);
    } else {
        try_threads();
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone());
}
