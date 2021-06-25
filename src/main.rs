mod parsing;
use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};


use std::thread::{self, JoinHandle};
use std::{time, vec};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;
use std::ops::Deref;
use std::time::{Duration, Instant};

const NOTIFY_FRECUENCY: u64 = 1;
const MIN_TIME_REQUESTS: u64 = 0;
const MAX_CONCURRENCY: isize = 5;
const MAX_PAGES: i32 = 3;

static mut SEM_COUNT: i32 = 3;
static mut COUNT: i32 = 0;

struct Page {
    word: Arc<String>,
    id: i32,
    sem: Arc<Semaphore>,
    condvar: Arc<(Mutex<std::time::Instant>, Condvar)>
}

impl Page {
    fn new(word: Arc<String>, id: i32, sem: Arc<Semaphore>, condvar: Arc<(Mutex<std::time::Instant>, Condvar)>) -> Page {
        Page {
            word: word,
            id: id,
            sem: sem,
            condvar: condvar
        }
    }

    fn concurrent_request(self) {
        println!("Palabra {:?} pagina {:?} intentando tomar el semaforo", self.word, self.id);
        self.sem.acquire();
        println!("<-- Haciendo request de sinónimos de {:?} en página {:?} -->", self.word, self.id);
        thread::sleep(Duration::from_millis(10000));
        self.sem.release();
        println!("##### La palabra {:?} termino de hacer el request en la página {:?} -->", self.word, self.id);
    }

    fn blocking_request(self) {
        let (lock, cvar) = &*self.condvar;
        let mut last = lock.lock().unwrap();

        loop {
            /* https://doc.rust-lang.org/nightly/std/sync/struct.Condvar.html#method.wait_timeout */
            // A notify is sent every NOTIFY_FREQUENCY seconds
            let timeout = time::Duration::from_millis(NOTIFY_FRECUENCY);

            let result = cvar.wait_timeout(last, timeout).unwrap();

            /* Si llegamos hasta acá es porque alguien le hizo notify() o porque se cumplió el timeout
             * de 3000 milis.
             */
            let now = time::Instant::now();

            last = result.0;

            /* Si pasaron más de MIN_TIME_REQUESTS salimos del loop */
            if now.duration_since(*last).as_secs() >= MIN_TIME_REQUESTS || MIN_TIME_REQUESTS == 0 {
                println!("HERE");
                break
            }
        }

        println!("Palabra {:?} pagina {:?} intentando tomar el semaforo", self.word, self.id);
        self.sem.acquire();
        println!("<-- Haciendo request de sinónimos de {:?} en página {:?} -->", self.word, self.id);
        thread::sleep(Duration::from_millis(10000));
        self.sem.release();
        println!("##### La palabra {:?} termino de hacer el request en la página {:?} -->", self.word, self.id);

        // Dejamos el último instante en que se ejecutó
        *last = time::Instant::now();

        cvar.notify_all();
    }

    fn request(self) {
        if (MIN_TIME_REQUESTS == 0) {
            self.concurrent_request();
        }
        else {
            self.blocking_request();
        }
    }
}


struct Controller {
    words: Arc<Vec<String>>,
    words_t: Vec<Word>,
    word_threads: Vec<JoinHandle<()>>,
    condvars: Arc<Vec<Arc<(Mutex<Instant>, Condvar)>>>,
    sem: Arc<Semaphore>
}

impl Controller {
    fn new(words: Arc<Vec<String>>) -> Controller {
        Controller {
            words: words,
            word_threads: vec![],
            condvars: Arc::new(vec!(
                // deberían iniciar en 0
                Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
                Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
                Arc::new((Mutex::new(time::Instant::now()), Condvar::new()))
            )),
            sem: Arc::new(Semaphore::new(MAX_CONCURRENCY)),
            words_t: vec!()
        }
    }

    fn spawn_word_threads(&mut self) -> () {
        for i in 0..self.words.len() {
            let current_w = Arc::new(self.words[i].clone());
            let sem = self.sem.clone();
            let cvs = self.condvars.clone();

            let mut worddd = Word::new(current_w, sem, cvs);

            self.word_threads.push(worddd.spawn_thread());
        }
    }

    fn join_word_threads(self) -> () {
        for word_thread in self.word_threads {
            let _ = word_thread.join();
        }
    }
}

struct Word {
    word: Arc<String>,
    pages_threads: Vec<JoinHandle<()>>,
    sem: Arc<Semaphore>,
    condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>
}

impl Word {
    fn new(word: Arc<String>, sem: Arc<Semaphore>, condvars: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>) -> Word {
        Word {
            word: word,
            sem: sem,
            condvars: condvars,
            pages_threads: vec!()
        }
    }

    fn spawn_pages_threads(&mut self) {
        for i in 0..MAX_PAGES {
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

    fn spawn_thread(mut self) -> JoinHandle<()>{
        thread::spawn(move || {
            self.spawn_pages_threads();
            self.join_pages_threads();
        })
    }

}



fn main() {

    let words = Arc::new(vec!(
        "palabra 1".to_string(),
        "palabra 2".to_string(),
        "palabra 3".to_string(),
        "palabra 4".to_string(),
        "palabra 5".to_string(),
        "palabra 6".to_string(),
        "palabra 7".to_string(),
    ));

    let mut controller = Controller::new(words);

    controller.spawn_word_threads();
    controller.join_word_threads();
}
