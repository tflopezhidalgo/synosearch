#[path = "../utils/counter.rs"]
mod counter;
#[path = "../threading/page_worker.rs"]
mod page;

use crate::main_threads::Parser;
use crate::Logger;
use counter::Counter;
use page::PageWorker;
use std::fmt::Display;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std_semaphore::Semaphore;

/// Handles the thread of each word
/// Spawns the thread for each page inside the word and controls the concurrency between them
pub struct WordWorker {
    /// The word whose synonyms are to find
    word: Arc<String>,
    /// The threads to process the synonym search for each word concurrently
    page_threads: Vec<JoinHandle<Vec<String>>>,
    /// The condition variables for each page
    condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>,
    /// The semaphore that limits the maximum amount of concurrent requests
    sem: Arc<Semaphore>,
    providers: Arc<Vec<Box<dyn Parser + Send + Sync>>>,
    logger: Arc<Logger>,
    min_time_request_sec: u64,
}

impl Display for WordWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[WordWorker][{}]", self.word)
    }
}

impl WordWorker {
    /// Returns a Word with the arguments given
    /// * word: The word whose synonyms are to find
    /// * condvars: The condition variables for each page
    /// * sem: The semaphore that limits the maximum amount of concurrent requests
    pub fn new(
        word: Arc<String>,
        condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>,
        sem: Arc<Semaphore>,
        providers: Arc<Vec<Box<dyn Parser + Send + Sync>>>,
        logger: Arc<Logger>,
        min_time_request_sec: u64,
    ) -> WordWorker {
        WordWorker {
            word: word,
            sem: sem,
            condvars: condvars,
            page_threads: vec![],
            providers: providers,
            logger: logger,
            min_time_request_sec: min_time_request_sec,
        }
    }

    /// Creates a thread for sending a request to each page and waits for all of them to finish
    pub fn send_requests_to_pages_concurrently(mut self) {
        self.spawn_pages_threads();
        self.join_pages_threads();
    }

    /// Creates a thread for processing each page
    fn spawn_pages_threads(&mut self) {
        for i in 0..self.providers.len() {
            let word_clone = self.word.clone();
            let condvar_clone = self.condvars[i as usize].clone();
            let sem_clone = self.sem.clone();
            let providers_clone = self.providers.clone();
            let logger_clone = self.logger.clone();

            self.logger
                .info(format!("{} Spawning thread for {:?} page", self, i));
            let page = PageWorker::new(
                word_clone,
                i as usize,
                condvar_clone,
                sem_clone,
                providers_clone,
                logger_clone,
                self.min_time_request_sec,
            );
            self.page_threads
                .push(thread::spawn(move || page.request()));
        }
    }

    /// Waits for each thread in page_threads to finish
    fn join_pages_threads(self) {
        self.logger.info(format!(
            "[{}] Joining threads for word: {}",
            self, self.word
        ));
        let mut synonyms = Vec::new();
        for page_thread in self.page_threads {
            synonyms.append(&mut page_thread.join().unwrap());
        }
        Counter::count(self.word.to_string(), synonyms, self.logger.clone());
    }
}
