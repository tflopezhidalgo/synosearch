use std::thread::{self, JoinHandle};
use std::{time, vec};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;
use std::ops::Deref;
use std::time::{Duration, Instant};

#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS: u64 = 1;
static MAX_CONCURRENCY: isize = 5;
static MAX_PAGES: i32 = 3;

fn main() {

    let p1 = controller::parsing::ThesaurusProvider {url: "".to_string()};
    let p2 = controller::parsing::YourDictionaryProvider {url: "".to_string()};
    let p3 = controller::parsing::MarianWebsterProvider {url: "".to_string()};

    let mut providers: Vec<Box<dyn controller::parsing::Parser>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    //let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for p in &providers {
        println!("{:?}", p.parse("car".to_string()));
    }

    let words = Arc::new(vec!(
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
    ));

    let mut controller = Controller::new(words, providers);

    controller.process_words_concurrently();
}
