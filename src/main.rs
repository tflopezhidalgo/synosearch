/*mod parsing;
use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};


fn main() {

    let p1 = &ThesaurusProvider {url: "".to_string()};
    let p2 = &YourDictionaryProvider {url: "".to_string()};
    let p3 = &MarianWebsterProvider {url: "".to_string()};

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for p in providers {
        println!("{:?}", p.parse("car".to_string()));
    }
}
*/

use std::io::Write;
use std::{thread};
use std::fs::File;
use std::sync::{Arc, Mutex};


fn main() {
    let f = match File::create("foo.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem creating the file: {:?}", error)
    };
    let logger = Arc::new(Mutex::new(f));

    let mut threads = Vec::new();
    for _ in 0..6 {
       let file = Arc::clone(&logger);

        let thread = thread::spawn(move || {
            let mut file = match file.lock() {
                Ok(m) => m,
                Err(error) => panic!("Problem lock file: {:?}", error)
            };
            match file.write_all(b"Hola mundo\n") {
                Ok(m) => m,
                Err(error) => panic!("Problem creating the file: {:?}", error)
            };


        });
        threads.push(thread);
    }

    for thread in threads {
        match thread.join() {
            Ok(m) => m,
            Err(error) => panic!("Problem creating the file: {:?}", error)
        };
    }
}