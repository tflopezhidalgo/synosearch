use std::thread::{self, JoinHandle};
use std::{time, vec};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;
use std::ops::Deref;
use std::time::Duration;

const REQ_TIMEOUT_SECS: u64 = 0;
const MAX_CONCURRENCY: isize = 10;
const MIN_WAITING_TIME: u64 =  0;

// TODO: La primera vez no deberían estar esperando el tiempo del timeout las pags
fn page(w: Arc<String>, id: i32, cv: Arc<(Mutex<(std::time::Instant, bool)>, Condvar)>) -> () {

    let (lock, cvar) = &*cv;

    { 
        let mut last = lock.lock().unwrap();

        loop {
            /* https://doc.rust-lang.org/nightly/std/sync/struct.Condvar.html#method.wait_timeout */
            let timeout = time::Duration::from_millis(MIN_WAITING_TIME);

            let result = cvar.wait_timeout(last, timeout).unwrap();

            let now = time::Instant::now();

            last = result.0;

            /* Si pasaron más de REQ_TIMEOUT_SECS salimos del loop */ 
            if (now.duration_since((*last).0).as_secs() > REQ_TIMEOUT_SECS) && !(*last).1 {
                break;
            }
        }
    }
    (*lock.lock().unwrap()).1 = true;

    println!("Requesting...({:?} {:?})", w, id);

    // Dejamos el último instante en que se ejecutó
    *lock.lock().unwrap() = (time::Instant::now(), false);

    cvar.notify_all();
}

fn word(w: Arc<String>, sem: Arc<Semaphore>, cvs: Arc<Vec<Arc<(Mutex<(std::time::Instant, bool)>, Condvar)>>>) {
    let mut pages: Vec<JoinHandle<()>> = vec!();

    for i in 0..1 {
        let w = w.clone();
        let sem = sem.clone();
        let cvs = cvs.clone();
        pages.push(
            thread::spawn(move || {
                page(w, i, cvs[i as usize].clone());
            })
        );
    }

    for page in pages {
        page.join();
    }
}

fn main() {
    let sem = Arc::new(Semaphore::new(MAX_CONCURRENCY));

    let cvs = Arc::new(vec!(
        // deberían iniciar en 0 
        Arc::new((Mutex::new((time::Instant::now() - time::Duration::from_secs(100000), false)), Condvar::new())),
        Arc::new((Mutex::new((time::Instant::now() - time::Duration::from_secs(100000), false)), Condvar::new())),
        Arc::new((Mutex::new((time::Instant::now() - time::Duration::from_secs(100000), false)), Condvar::new()))
    ));

    let words = Arc::new(vec!(
        1.to_string(),
        2.to_string(),
        3.to_string(),
        4.to_string(),
        5.to_string(),
        6.to_string(),
        7.to_string(),
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
