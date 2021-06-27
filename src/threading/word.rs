#[path = "../utils/counter.rs"]
mod counter;
#[path = "../threading/page.rs"]
mod page;

use crate::Logger;
use counter::Counter;
use page::Page;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std_semaphore::Semaphore;

/// Handles the thread of each word
/// Spawns the thread for each page inside the word and controls the concurrency between them
pub struct Word {
    /// The word whose synonyms are to find
    word: Arc<String>,
    /// The threads to process the synonym search for each word concurrently
    page_threads: Vec<JoinHandle<Vec<String>>>,
    /// The condition variables for each page
    condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>,
    /// The semaphore that limits the maximum amount of concurrent requests
    sem: Arc<Semaphore>,
    providers: Arc<Vec<Box<dyn crate::parsing::Parser + Send + Sync>>>,
    logger: Arc<Logger>,
}

impl Word {
    /// Returns a Word with the arguments given
    /// * word: The word whose synonyms are to find
    /// * condvars: The condition variables for each page
    /// * sem: The semaphore that limits the maximum amount of concurrent requests
    pub fn new(
        word: Arc<String>,
        condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>,
        sem: Arc<Semaphore>,
        providers: Arc<Vec<Box<dyn crate::parsing::Parser + Send + Sync>>>,
        logger: Arc<Logger>,
    ) -> Word {
        Word {
            word: word,
            sem: sem,
            condvars: condvars,
            page_threads: vec![],
            providers: providers,
            logger: logger,
        }
    }

    /// Creates a thread for sending a request to each page and waits for all of them to finish
    pub fn send_requests_to_pages_concurrently(mut self) {
        self.logger
            .write("INFO: Spawn words threads Words\n".to_string());
        self.spawn_pages_threads();
        self.logger
            .write("INFO: Join words threads Words\n".to_string());
        self.join_pages_threads();
    }

    /// Creates a thread for processing each page
    fn spawn_pages_threads(&mut self) {
        for i in 0..crate::MAX_PAGES {
            let word_clone = self.word.clone();
            let condvar_clone = self.condvars[i as usize].clone();
            let sem_clone = self.sem.clone();
            let providers_clone = self.providers.clone();
            let logger_clone = self.logger.clone();

            self.logger
                .write("INFO: Send request to page threads\n".to_string());
            let page = Page::new(
                word_clone,
                i as usize,
                condvar_clone,
                sem_clone,
                providers_clone,
                logger_clone,
            );
            self.page_threads
                .push(thread::spawn(move || page.request()));
        }
    }

    /// Waits for each thread in page_threads to finish
    fn join_pages_threads(self) {
        self.logger
            .write(format!("INFO: Join threads from word: {}\n", self.word));
        let mut synonimous = Vec::new();
        for page_thread in self.page_threads {
            synonimous.append(&mut page_thread.join().unwrap());
        }
        self.logger.write(format!(
            "INFO: Get all synonimous from word {} and count",
            self.word
        ));
        Counter::count(self.word.to_string(), synonimous, self.logger.clone());
    }
}
