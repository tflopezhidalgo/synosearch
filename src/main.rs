use std::thread::{self, JoinHandle};
use std::{time, vec};
use std_semaphore::Semaphore;
use std::sync::{Arc, Mutex, Condvar};


fn pagina(w: Arc<String>, id: i32, sem: Arc<Semaphore>, condvar: Arc<(Mutex<std::time::Instant>, Condvar)>) -> () {
    sem.acquire();

    let (lock, cvar) = &*condvar;
    let mut last = cvar.wait_while(
        lock.lock().unwrap(), 
        |last| { 
            let now = time::Instant::now();
            println!("hey, {}", id);
            now.duration_since(*last).as_secs() < 10
        }).unwrap();

    println!(" ----> haciendo request para sinónimos de {:?} en página {:?}", w, id);
    let duration = time::Duration::from_millis(0);

    thread::sleep(duration);
    sem.release();

    *last = time::Instant::now();

    cvar.notify_all();

    println!("Devolviendo ({:?})...", id);
}

fn palabra(w: Arc<String>, sem: Arc<Semaphore>, condvar: Arc<(Mutex<std::time::Instant>, Condvar)>) {
    println!("Buscando sinónimos para palabra: {:?}", w);

    let mut paginas : Vec<JoinHandle<()>> = vec!();

    for i in 0..2 {
        let w = w.clone();
        let sem = sem.clone();
        let condvar = condvar.clone();
        paginas.push(
            thread::spawn(move || {
                pagina(w, i, sem, condvar);
            })
        );
    }

    for pagina in paginas {
        pagina.join();
    }
}

fn main() {
    let sem = Arc::new(Semaphore::new(3));

    let pair = Arc::new((Mutex::new(time::Instant::now()), Condvar::new()));

    let words = Arc::new(vec!(
        "palabra 1".to_string(),
        "palabra 2".to_string(),
        "palabra 3".to_string(),
        "palabra 4".to_string(),
        "palabra 5".to_string(),
        "palabra 6".to_string(),
    ));

    let mut w_threads : Vec<JoinHandle<()>> = vec!();

    for i in 0..words.len() {

        let current_w = Arc::new(words[i].clone());
        let sem = sem.clone();
        let pair2 = pair.clone();

        w_threads.push(thread::spawn(move || {
            palabra(current_w.clone(), sem, pair2);
        }));
    }

    for t in w_threads { 
        t.join();
    }
}
