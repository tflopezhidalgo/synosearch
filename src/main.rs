use std::{vec};
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

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS: u64 = 1;
static MAX_CONCURRENCY: isize = 5;
static MAX_PAGES: i32 = 3;

fn choose_mode(mode:String, filename: String) {
    //let words = FileReader::new(filename).get_words();

    if mode.eq("actors") {
        println!("actors");
    } else if mode.eq("threads") {
        println!("threads");
        run_parsers();
    } else {

    }
}

fn run_parsers() {
    let p1 = ThesaurusProvider;
    let p2 = YourDictionaryProvider;
    let p3 = MarrianWebsterProvider;

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words = Arc::new(vec!(
        "car".to_string()/*,
        "bus".to_string(),
        "paper".to_string(),
        "love".to_string(),
        "computer".to_string(),
        "key".to_string(),
        "person".to_string(),*/
    ));

    let controller = Controller::new(words, providers_arc);

    controller.process_words_concurrently();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone())
}