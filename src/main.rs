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
use std::time::Duration;

const NOTIFY_FRECUENCY: u64 = 1;
const MIN_TIME_REQUESTS: u64 = 0;
const MAX_CONCURRENCY: isize = 5;
const MAX_PAGES: i32 = 3;

static mut SEM_COUNT: i32 = 3;
static mut COUNT: i32 = 0;


fn page(w: Arc<String>, id: i32, sem: Arc<Semaphore>, cv: Arc<(Mutex<std::time::Instant>, Condvar)>) -> () {

    println!("WORD: {} \t PAGE: {}", w, id);
    println!("----> WORD: {} \t PAGE: {}", w, id);

    if(MIN_TIME_REQUESTS == 0)
    {
        println!("Palabra {:?} pagina {:?} intentando tomar el semaforo", w, id);
        sem.acquire();
        println!("<-- Haciendo request de sinónimos de {:?} en página {:?} -->", w, id);
        thread::sleep(Duration::from_millis(10000));
        sem.release();
        println!("##### La palabra {:?} termino de hacer el request en la página {:?} -->", w, id);

    }
    else {
        let (lock, cvar) = &*cv;
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

        println!("Palabra {:?} pagina {:?} intentando tomar el semaforo", w, id);
        sem.acquire();
        println!("<-- Haciendo request de sinónimos de {:?} en página {:?} -->", w, id);
        thread::sleep(Duration::from_millis(10000));
        sem.release();
        println!("##### La palabra {:?} termino de hacer el request en la página {:?} -->", w, id);

        // Dejamos el último instante en que se ejecutó
        *last = time::Instant::now();

        cvar.notify_all();
    }

}

fn word(w: Arc<String>, sem: Arc<Semaphore>, cvs: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>) {
    println!("Buscando sinónimos para palabra: {:?}", w);

    let mut paginas : Vec<JoinHandle<()>> = vec!();

    for i in 0..MAX_PAGES {
        let w = w.clone();
        let sem = sem.clone();
        let cvs = cvs.clone();
        paginas.push(
            thread::spawn(move || {
                page(w, i, sem, cvs[i as usize].clone());
            })
        );
    }

    for pagina in paginas {
        pagina.join();
    }
}

fn main() {
    let sem = Arc::new(Semaphore::new(MAX_CONCURRENCY));

    let cvs = Arc::new(vec!(
        // deberían iniciar en 0
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
        Arc::new((Mutex::new(time::Instant::now()), Condvar::new())),
    ));

    let words = Arc::new(vec!(
        "palabra 1".to_string(),
        "palabra 2".to_string(),
        "palabra 3".to_string(),
        "palabra 4".to_string(),
        "palabra 5".to_string(),
        "palabra 6".to_string(),
        "palabra 7".to_string(),
    ));

    let mut w_threads : Vec<JoinHandle<()>> = vec!();

    for i in 0..words.len() {

        let current_w = Arc::new(words[i].clone());
        let sem = sem.clone();
        let cvs = cvs.clone();

        w_threads.push(thread::spawn(move || {
            word(current_w.clone(), sem, cvs);
        }));
    }

    for t in w_threads { 
        t.join();
    }
}
