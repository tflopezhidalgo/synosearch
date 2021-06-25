#[path = "../threading/word.rs"] mod word;
use word::Word;
use std::sync::{Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use std_semaphore::Semaphore;
use std::{time, vec};
use std::time::{Duration, Instant};

pub struct Controller {
    words: Arc<Vec<String>>,
    words_t: Vec<Word>,
    word_threads: Vec<JoinHandle<()>>,
    condvars: Arc<Vec<Arc<(Mutex<Instant>, Condvar)>>>,
    sem: Arc<Semaphore>
}

impl Controller {
    pub fn new(words: Arc<Vec<String>>) -> Controller {
        Controller {
            words: words,
            word_threads: vec![],
            condvars: Arc::new(vec!(
                // deberían iniciar en 0
                Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
                Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
                Arc::new((Mutex::new(time::Instant::now()), Condvar::new()))
            )),
            sem: Arc::new(Semaphore::new(crate::MAX_CONCURRENCY)),
            words_t: vec!()
        }
    }

    pub fn spawn_word_threads(&mut self) -> () {
        for i in 0..self.words.len() {
            let current_w = Arc::new(self.words[i].clone());
            let sem = self.sem.clone();
            let cvs = self.condvars.clone();

            let mut worddd = Word::new(current_w, sem, cvs);

            self.word_threads.push(worddd.spawn_thread());
        }
    }

    pub fn join_word_threads(self) -> () {
        for word_thread in self.word_threads {
            let _ = word_thread.join();
        }
    }
}
