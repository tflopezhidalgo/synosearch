use std::thread::{self, JoinHandle};
use std::{time, vec};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;

const REQ_TIMEOUT_SECS: u64 = 3;

const MAX_CONCURRENCY: isize = 3;


fn pagina(w: Arc<String>, id: i32, sem: Arc<Semaphore>, cv: Arc<(Mutex<std::time::Instant>, Condvar)>) -> () {
    sem.acquire();

    let (lock, cvar) = &*cv;

    let mut last = lock.lock().unwrap();

    loop {

        /* https://doc.rust-lang.org/nightly/std/sync/struct.Condvar.html#method.wait_timeout */
        let timeout = time::Duration::from_millis(3000);

        let result = cvar.wait_timeout(last, timeout).unwrap();

        let now = time::Instant::now();

        last = result.0; 

        /* Si pasaron más de REQ_TIMEOUT_SECS salimos del loop */ 
        if now.duration_since(*last).as_secs() > REQ_TIMEOUT_SECS {
            break
        }
    }

    println!("<-- Haciendo request de sinónimos de {:?} en página {:?} -->", w, id);

    sem.release();

    // Dejamos el último instante en que se ejecutó
    *last = time::Instant::now();

    cvar.notify_all();
}

fn palabra(w: Arc<String>, sem: Arc<Semaphore>, cvs: Arc<Vec<Arc<(Mutex<std::time::Instant>, Condvar)>>>) {
    println!("Buscando sinónimos para palabra: {:?}", w);

    let mut paginas : Vec<JoinHandle<()>> = vec!();

    for i in 0..5 {
        let w = w.clone();
        let sem = sem.clone();
        let cvs = cvs.clone();
        paginas.push(
            thread::spawn(move || {
                pagina(w, i, sem, cvs[i as usize].clone());
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
            palabra(current_w.clone(), sem, cvs);
        }));
    }

    for t in w_threads { 
        t.join();
    }
}
