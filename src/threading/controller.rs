#[path = "../threading/word_worker.rs"]
mod word;
use word::WordWorker;

use super::parsing::parser::Parser;

use crate::Logger;
use std::fmt::Display;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::{time, vec};
use std_semaphore::Semaphore;

/// Handles the main thread
/// Spawns the thread for each word and controls the concurrency between them
pub struct Controller {
    /// The words whose synonyms are to find
    words: Arc<Vec<String>>,
    /// The threads to process the synonym search for each word concurrently
    word_threads: Vec<JoinHandle<()>>,
    /// The condition variables for each page
    condvars: Arc<Vec<Arc<(Mutex<Instant>, Condvar)>>>,
    /// The semaphore that limits the maximum amount of concurrent requests
    sem: Arc<Semaphore>,
    providers: Arc<Vec<Box<dyn Parser + Send + Sync>>>,
    logger: Arc<Logger>,
    min_time_request_sec: u64
}

impl Display for Controller {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Controller")
    }
}

impl Controller {
    /// Returns a Controller with the arguments given
    /// * words: The words whose synonyms are to find
    pub fn new(
        words: Arc<Vec<String>>,
        providers: Arc<Vec<Box<dyn Parser + Send + Sync>>>,
        logger: Arc<Logger>,
        max_concurrency: usize,
        min_time_request_sec: u64
    ) -> Controller {
        let max_pages = providers.len();
        Controller {
            words: words,
            word_threads: vec![],
            condvars: Controller::init_condvars(max_pages),
            sem: Arc::new(Semaphore::new(max_concurrency as isize)),
            providers: providers,
            logger: logger,
            min_time_request_sec: min_time_request_sec
        }
    }

    /// Initializes the condvar for each page from 0 to MAX_PAGES
    fn init_condvars(max_pages: usize) -> Arc<Vec<Arc<(Mutex<Instant>, Condvar)>>> {
        let mut condvars = vec![];
        for _ in 0..max_pages {
            condvars.push(Arc::new((Mutex::new(time::Instant::now()), Condvar::new())));
        }
        return Arc::from(condvars);
    }

    /// Creates a thread for processing each word and waits for all of them to finish
    pub fn process_words_concurrently(mut self) {
        self.logger.info(format!("[{}] Spawn words threads", self));
        self.spawn_word_threads();
        self.logger.info(format!("[{}] Join words threads", self));
        self.join_word_threads();
    }

    /// Creates a thread for processing each word
    fn spawn_word_threads(&mut self) {
        for i in 0..self.words.len() {
            let word_clone = Arc::new(self.words[i].clone());
            let condvars_clone = self.condvars.clone();
            let sem_clone = self.sem.clone();
            let providers_clone = self.providers.clone();
            let logger_clone = self.logger.clone();

            let word = WordWorker::new(
                word_clone,
                condvars_clone,
                sem_clone,
                providers_clone,
                logger_clone,
                self.min_time_request_sec,
            );

            self.logger
                .info("Send request to words threads".to_string());
            self.word_threads.push(thread::spawn(move || {
                word.send_requests_to_pages_concurrently();
            }));
        }
    }

    /// Waits for each thread in word_threads to finish
    fn join_word_threads(self) -> () {
        for word_thread in self.word_threads {
            let _ = word_thread.join();
        }
    }
}
