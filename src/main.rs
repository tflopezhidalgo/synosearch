mod parsing;
use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};


use std::thread::{self, JoinHandle};
use std_semaphore::Semaphore;
use std::sync::Arc;
use std::collections::VecDeque;


fn pagina(w: Arc<String>, id: i32) -> () {
    println!("Buscando sinónimos de {:?} en página {:?}", w, id);
}

fn palabra(w: Arc<String>) {
    println!("Buscando sinónimos para palabra: {:?}", w);

    let mut paginas : Vec<JoinHandle<()>> = vec!();

    for i in 0..2 {
        let w = w.clone();
        paginas.push(
            thread::spawn(move || {
                pagina(w, i);
            })
        );
    }

    for pagina in paginas {
        pagina.join();
    }
}

fn main() {

    let p1 = &ThesaurusProvider {url: "".to_string()};
    let p2 = &YourDictionaryProvider {url: "".to_string()};
    let p3 = &MarianWebsterProvider {url: "".to_string()};

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for p in providers {
        println!("{:?}", p.parse("car".to_string()));
    }
    //let sem = Arc::new(Semaphore::new(3));

    let words = Arc::new(vec!(
        "palabra 1".to_string(),
        "palabra 2".to_string()
    ));

    let mut w_threads : Vec<JoinHandle<()>> = vec!();

    for i in 0..words.len() {

        let current_w = Arc::new(words[i].clone());

        w_threads.push(thread::spawn(move || {
            palabra(current_w.clone());
        }));
    }

    for t in w_threads { 
        t.join();
    }
}
