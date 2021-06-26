
use std::sync::{Arc};
use std::process;
use std::env;


#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

mod parsing;
use parsing::{
    ThesaurusProvider,
    YourDictionaryProvider,
    MarrianWebsterProvider,
    Parser
};


#[path = "utils/logger.rs"] mod logger;
use logger::Logger;
#[path = "utils/file_reader.rs"] mod file_reader;
use file_reader::FileReader;

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS: u64 = 1;
static MAX_CONCURRENCY: isize = 5;
static MAX_PAGES: i32 = 3;
const FILENAME: &str = "src/log.txt";

fn choose_mode(mode:String, filename: String) {
    let logger = Arc::from(Logger::new(FILENAME));

    let words = FileReader::new(filename, logger.clone()).get_words();

    if mode.eq("actors") {
        println!("actors");
    } else if mode.eq("threads") {
        println!("threads");
        logger.write("INFO: Run program mod threads\n".to_string());
        run_parsers(words, logger.clone());
    } else {

    }
}

fn run_parsers(words: Vec<String>, logger: Arc<Logger>) {
    let p1 = ThesaurusProvider::new(logger.clone());
    let p2 = YourDictionaryProvider::new(logger.clone());
    let p3 = MarrianWebsterProvider::new(logger.clone());

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words_arc = Arc::from(words);

    let controller = Controller::new(words_arc, providers_arc);

    controller.process_words_concurrently();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone())
}