#[path = "../threading/word.rs"] mod word;

use word::Word;
use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use std_semaphore::Semaphore;
use std::{time, vec};
use std::time::{Instant};

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
    providers: Arc<Vec<Box<dyn crate::parsing::Parser + Send + Sync>>>
}

impl Controller {
    /// Returns a Controller with the arguments given
    /// * words: The words whose synonyms are to find
    pub fn new(words: Arc<Vec<String>>, providers: Arc<Vec<Box<dyn crate::parsing::Parser + Send + Sync>>>) -> Controller {
        Controller {
            words: words,
            word_threads: vec![],
            condvars: Controller::init_condvars(),
            sem: Arc::new(Semaphore::new(crate::MAX_CONCURRENCY)),
            providers: providers
        }
    }

    /// Initializes the condvar for each page from 0 to MAX_PAGES
    fn init_condvars() -> Arc<Vec<Arc<(Mutex<Instant>, Condvar)>>> {
        let mut condvars = vec!();
        for _ in 0..crate::MAX_PAGES {
            condvars.push(Arc::new((Mutex::new(time::Instant::now()), Condvar::new())));
        }
        return Arc::from(condvars);
    }

    /// Creates a thread for processing each word and waits for all of them to finish
    pub fn process_words_concurrently(mut self) {
        self.spawn_word_threads();
        self.join_word_threads();
    }

    /// Creates a thread for processing each word
    fn spawn_word_threads(&mut self) {
        for i in 0..self.words.len() {
            let word_clone = Arc::new(self.words[i].clone());
            let condvars_clone = self.condvars.clone();
            let sem_clone = self.sem.clone();
            let providers_clone = self.providers.clone();

            let word = Word::new(word_clone, condvars_clone, sem_clone, providers_clone);
            self.word_threads.push(
                thread::spawn(move || {
                    word.send_requests_to_pages_concurrently();
                })
            );
        }
    }

    /// Waits for each thread in word_threads to finish
    fn join_word_threads(self) -> () {
        for word_thread in self.word_threads {
            let _ = word_thread.join();
        }
    }
}
