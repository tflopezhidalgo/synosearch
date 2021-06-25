use std::{vec};
use std::sync::{Arc};

#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

mod parsing;
use parsing::{
    ThesaurusProvider,
    YourDictionaryProvider,
    MarianWebsterProvider,
    Parser
};

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS: u64 = 1;
static MAX_CONCURRENCY: isize = 5;
static MAX_PAGES: i32 = 3;

fn main() {

    let p1 = ThesaurusProvider {url: "".to_string()};
    let p2 = YourDictionaryProvider {url: "".to_string()};
    let p3 = MarianWebsterProvider {url: "".to_string()};

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    //let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

   /* for p in &providers {
        println!("{:?}", p.parse("car".to_string()));
    }*/

    let words = Arc::new(vec!(
        "car".to_string(),
        "bus".to_string(),
        "paper".to_string(),
        "love".to_string(),
        "computer".to_string(),
        "key".to_string(),
        "person".to_string(),
    ));

    let controller = Controller::new(words, providers_arc);

    controller.process_words_concurrently();
}
