mod parsing;
use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};


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

    let words = Arc::new(vec!(
        "palabra 1".to_string(),
        "palabra 2".to_string(),
        "palabra 3".to_string(),
        "palabra 4".to_string(),
        "palabra 5".to_string(),
        "palabra 6".to_string(),
        "palabra 7".to_string(),
    ));

    let mut controller = Controller::new(words);

    controller.spawn_word_threads();
    controller.join_word_threads();
}
