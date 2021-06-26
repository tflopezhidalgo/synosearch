use std::collections::HashMap;

/* COUNTER */

pub struct Counter;

impl Counter {
    pub fn count(word: String, synonimous: Vec<String>) { 
        let mut synonimous_counter = HashMap::new();
        for s in synonimous {
            let entry = synonimous_counter.entry(s).or_insert(0);
            *entry += 1;
        }

        /*
        for c in synonimous_counter {
            println!("{:?}", c);
        }
        */

        println!("WORD: {}\nSINONIMOUS: \n{:?}\n", word, synonimous_counter);
    }
}