use std::sync::Arc;
#[path = "threading/controller.rs"]
mod controller;
use controller::Controller;

#[path = "parsing/mod.rs"]
mod parsing;
use parsing::parser::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};

use crate::Logger;

pub fn main_threads(words: Vec<String>, logger: Arc<Logger>, max_concurrency: usize,
        min_time_request_sec: u64) {
    let p1 = ThesaurusProvider::new(logger.clone());
    let p2 = YourDictionaryProvider::new(logger.clone());
    let p3 = MerriamWebsterProvider::new(logger.clone());

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words_arc = Arc::from(words);

    let controller = Controller::new(words_arc, providers_arc, logger, max_concurrency, min_time_request_sec);

    controller.process_words_concurrently();
}