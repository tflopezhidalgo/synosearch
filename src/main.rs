use std::sync::{Arc};

#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

mod parsing;
mod utils;

use utils::{
    Counter,
    FileReader,
    Logger
};

use parsing::{
    ThesaurusProvider,
    YourDictionaryProvider,
    MarrianWebsterProvider,
    Parser
};

use std::{thread};
use std::process;
use std::env;
static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS: u64 = 1;
static MAX_CONCURRENCY: isize = 5;
static MAX_PAGES: i32 = 3;
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

    /*for w in &words {
        let mut synonimous = Vec::new();
        println!("WORD: {}", w);
        for p in &providers {
            synonimous.append(&mut p.parse(w.to_string()));
        }
        Counter::count(synonimous);
    }*/

    let p1 = ThesaurusProvider;
    let p2 = YourDictionaryProvider;
    let p3 = MarrianWebsterProvider;

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words2 = Arc::new(vec!(
        "car".to_string(),
        "bus".to_string(),
        "paper".to_string(),
        "love".to_string(),
        "computer".to_string(),
        "key".to_string(),
        "person".to_string(),
    ));
    //let words_arc = Arc::from(words.clone());

    let controller = Controller::new(words2, providers_arc);

    controller.process_words_concurrently();
}

fn choose_mode(mode:String, filename: String) {
    let words = FileReader::new(filename).get_words();

    if mode.eq("actors") {
        println!("MODE: \t Actors");
    } else if mode.eq("threads") {
        println!("MODE: \t Threads");
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
