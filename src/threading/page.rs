use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, Condvar};
use std_semaphore::Semaphore;
use std::{time, vec};
use std::time::{Duration, Instant};

pub struct Page {
    word: Arc<String>,
    id: i32,
    sem: Arc<Semaphore>,
    condvar: Arc<(Mutex<std::time::Instant>, Condvar)>
}

impl Page {
    pub fn new(word: Arc<String>, id: i32, sem: Arc<Semaphore>, condvar: Arc<(Mutex<std::time::Instant>, Condvar)>) -> Page {
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
            let timeout = time::Duration::from_millis(crate::NOTIFY_FRECUENCY);

            let result = cvar.wait_timeout(last, timeout).unwrap();

            /* Si llegamos hasta acá es porque alguien le hizo notify() o porque se cumplió el timeout
             * de 3000 milis.
             */
            let now = time::Instant::now();

            last = result.0;

            /* Si pasaron más de MIN_TIME_REQUESTS salimos del loop */
            if now.duration_since(*last).as_secs() >= crate::MIN_TIME_REQUESTS || crate::MIN_TIME_REQUESTS == 0 {
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

    pub fn request(self) {
        if (crate::MIN_TIME_REQUESTS == 0) {
            self.concurrent_request();
        }
        else {
            self.blocking_request();
        }
    }
}