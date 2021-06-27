use std::thread::{self};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;
use std::{time};
use std::time::{Duration};


pub struct Page {
    /// The word whose synonyms are to find
    word: Arc<String>,
    /// The id of the page
    id: usize,
    /// The condition variable for the page
    condvar: Arc<(Mutex<std::time::Instant>, Condvar)>,
    /// The semaphore that limits the maximum amount of concurrent requests
    sem: Arc<Semaphore>,
    providers: Arc<Vec<Box<dyn crate::parsing::Parser + Send + Sync>>>
}

impl Page {
    /// Returns a Page with the arguments given
    /// * word: The word whose synonyms are to find
    /// * id: The id of the page
    /// * condvar: The condition variable for the page
    /// * sem: The semaphore that limits the maximum amount of concurrent requests
    pub fn new(word: Arc<String>,
               id: usize,
               condvar: Arc<(Mutex<std::time::Instant>, Condvar)>,
               sem: Arc<Semaphore>,
               providers: Arc<Vec<Box<dyn crate::parsing::Parser + Send + Sync>>>) -> Page {
        Page {
            word: word,
            id: id,
            sem: sem,
            condvar: condvar,
            providers: providers
        }
    }

    /// Sends a request
    fn send_request(&self) -> Vec<String> {
        println!("WORD {:?} \t PAGE {:?} \t TRYING TO DO A REQUEST", self.word, self.id);
        self.sem.acquire();
        println!("WORD {:?} \t PAGE {:?} \t DOING REQUEST ---------------", self.word, self.id);
        let word_clone = self.word.clone();

        let vec = self.providers[self.id].parse(word_clone.to_string());
        println!("\nWORD {:?} \t PAGE {:?} \t SYNONYMS: {:?}", self.word, self. id, vec);

        thread::sleep(Duration::from_millis(10000));
        self.sem.release();
        println!("WORD {:?} \t PAGE {:?} \t FINISHED REQUEST", self.word, self.id);
        return vec;
    }

    /// Handles the request when more than one request per page can occur at a time
    fn concurrent_request(self) -> Vec<String>{
        self.send_request()
    }

    /// Handles the request when at most one request per page can occur at a time
    fn blocking_request(self) -> Vec<String> {
        let (lock, cvar) = &*self.condvar;
        let mut last = lock.lock().unwrap();

        loop {
            /* https://doc.rust-lang.org/nightly/std/sync/struct.Condvar.html#method.wait_timeout */
            // A notify is sent every NOTIFY_FREQUENCY seconds
            let timeout = time::Duration::from_millis(crate::NOTIFY_FRECUENCY);
            let result = cvar.wait_timeout(last, timeout).unwrap();

            // At this point a notify() has been made or a timeout has occured
            let now = time::Instant::now();

            last = result.0;

            // Condition to go out of the loop
            if now.duration_since(*last).as_secs() >= crate::MIN_TIME_REQUESTS_SECS  {
                break
            }
        }

        let vec = self.send_request();
        *last = time::Instant::now();
        cvar.notify_all();
        return vec;
    }

    /// Handles the request to a page
    pub fn request(self) -> Vec<String> {
        if crate::MIN_TIME_REQUESTS_SECS == 0 {
            return self.concurrent_request();
        }
        else {
            return self.blocking_request();
        }
    }
}