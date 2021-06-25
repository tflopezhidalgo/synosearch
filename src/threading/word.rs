#[path = "../threading/page.rs"] mod page;
use page::Page;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;

/// Handles the thread of each word
/// Spawns the thread for each page inside the word and controls the concurrency between them
pub struct Word {
    /// The word whose synonyms are to find
    word: Arc<String>,
    /// The threads to process the synonym search for each word concurrently
    page_threads: Vec<JoinHandle<()>>,
    /// The condition variables for each page
    condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>,
    /// The semaphore that limits the maximum amount of concurrent requests
    sem: Arc<Semaphore>
}

impl Word {
    /// Returns a Word with the arguments given
    /// * word: The word whose synonyms are to find
    /// * condvars: The condition variables for each page
    /// * sem: The semaphore that limits the maximum amount of concurrent requests
    pub fn new(word: Arc<String>,
               condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>,
               sem: Arc<Semaphore>) -> Word {
        Word {
            word: word,
            sem: sem,
            condvars: condvars,
            page_threads: vec!()
        }
    }

    /// Creates a thread for sending a request to each page and waits for all of them to finish
    pub fn send_requests_to_pages_concurrently(mut self) {
        self.spawn_pages_threads();
        self.join_pages_threads();
    }

    /// Creates a thread for processing each page
    fn spawn_pages_threads(&mut self) {
        for i in 0..crate::MAX_PAGES {
            let word_clone = self.word.clone();
            let condvar_clone = self.condvars[i as usize].clone();
            let sem_clone = self.sem.clone();

            let page = Page::new(word_clone, i, condvar_clone, sem_clone);
            self.page_threads.push(
                thread::spawn(move || {
                    page.request();
                })
            );
        }
    }

    /// Waits for each thread in page_threads to finish
    fn join_pages_threads(self) {
        for page_thread in self.page_threads {
            let _ = page_thread.join();
        }
    }
}