#[path = "../threading/page.rs"] mod page;
use page::Page;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;

pub struct Word {
    word: Arc<String>,
    pages_threads: Vec<JoinHandle<()>>,
    sem: Arc<Semaphore>,
    condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>
}

impl Word {
    pub fn new(word: Arc<String>, sem: Arc<Semaphore>, condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>) -> Word {
        Word {
            word: word,
            sem: sem,
            condvars: condvars,
            pages_threads: vec!()
        }
    }

    fn spawn_pages_threads(&mut self) {
        for i in 0..crate::MAX_PAGES {
            let w = self.word.clone();
            let sem = self.sem.clone();
            let cvs = self.condvars.clone();

            let p = Page::new(w, i, sem, cvs[i as usize].clone());
            self.pages_threads.push(
                thread::spawn(move || {

                    p.request();
                })
            );
        }
    }

    fn join_pages_threads(self) {
        for page_thread in self.pages_threads {
            let _ = page_thread.join();
        }
    }

    pub fn spawn_thread(mut self) -> JoinHandle<()>{
        thread::spawn(move || {
            self.spawn_pages_threads();
            self.join_pages_threads();
        })
    }

}