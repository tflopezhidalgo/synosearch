use super::Parser;

use crate::Logger;

use std::fmt::Display;
use std::sync::{Arc, Condvar, Mutex};
use std::time;
use std_semaphore::Semaphore;

const NOTIFY_FRECUENCY: u64 = 1;

pub struct PageWorker {
    /// The word whose synonyms are to find
    word: Arc<String>,
    /// The id of the page
    id: usize,
    /// The condition variable for the page
    condvar: Arc<(Mutex<std::time::Instant>, Condvar)>,
    /// The semaphore that limits the maximum amount of concurrent requests
    sem: Arc<Semaphore>,
    /// List of available Parsers
    providers: Arc<Vec<Box<dyn Parser + Send + Sync>>>,
    /// Reference to the global logger
    logger: Arc<Logger>,
    /// Minimum time between two consecutives request to the same site
    min_time_request_sec: u64,
}

impl Display for PageWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[PageWorker][{}][{}]", self.word, self.id)
    }
}

impl PageWorker {
    /// Returns a Page with the arguments given
    /// * word: The word whose synonyms are to find
    /// * id: The id of the page
    /// * condvar: The condition variable for the page
    /// * sem: The semaphore that limits the maximum amount of concurrent requests
    pub fn new(
        word: Arc<String>,
        id: usize,
        condvar: Arc<(Mutex<std::time::Instant>, Condvar)>,
        sem: Arc<Semaphore>,
        providers: Arc<Vec<Box<dyn Parser + Send + Sync>>>,
        logger: Arc<Logger>,
        min_time_request_sec: u64,
    ) -> PageWorker {
        PageWorker {
            word: word,
            id: id,
            sem: sem,
            condvar: condvar,
            providers: providers,
            logger: logger,
            min_time_request_sec: min_time_request_sec,
        }
    }

    fn send_request(&self) -> Vec<String> {
        self.logger.info(format!("{} Acquiring semaphore...", self));

        self.sem.acquire();

        self.logger.info(format!("{} Doing request...", self));

        let word_clone = self.word.clone();

        let vec = self.providers[self.id].parse(word_clone.to_string());

        self.logger
            .info(format!("{} Request result: {:?}", self, vec));

        self.sem.release();

        self.logger.info(format!("{} Released semaphore", self));

        vec
    }

    /// Handles the request when more than one request per page can occur at a time
    fn concurrent_request(self) -> Vec<String> {
        self.send_request()
    }

    /// Handles the request when at most one request per page can occur at a time
    fn blocking_request(self) -> Vec<String> {
        let (lock, cvar) = &*self.condvar;
        let mut guard = lock.lock().unwrap();

        self.logger.info(format!("{} Acquired lock", self));

        loop {
            /* https://doc.rust-lang.org/nightly/std/sync/struct.Condvar.html#method.wait_timeout */
            // A notify is sent every NOTIFY_FREQUENCY seconds
            let timeout = time::Duration::from_millis(NOTIFY_FRECUENCY);

            // Acá no podemos loggear por el loop (llenamos el log de ruido)
            let result = cvar.wait_timeout(guard, timeout).unwrap();
            // At this point a notify() has been made or a timeout has occured
            let now = time::Instant::now();
            guard = result.0;

            // Condition to go out of the loop
            if now.duration_since(*guard).as_secs() >= self.min_time_request_sec {
                break;
            }
        }
        let vec = self.send_request();
        *guard = time::Instant::now();

        cvar.notify_all();

        self.logger
            .info(format!("{} Notifying and releasing lock", self));

        vec
    }

    /// Handles the request to a page
    pub fn request(self) -> Vec<String> {
        if self.min_time_request_sec == 0 {
            return self.concurrent_request();
        }
        self.blocking_request()
    }
}
