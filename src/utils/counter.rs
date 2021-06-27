use crate::Arc;
use std::collections::HashMap;
use crate::Logger;

/* COUNTER */

pub struct Counter;

impl Counter {
    pub fn count(word: String, synonimous: Vec<String>, logger: Arc<Logger>) { 
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
        logger.write(format!("INFO: WORD: {}\nSINONIMOUS: \n{:?}\n", word, synonimous_counter));
        println!("WORD: {}\nSINONIMOUS: \n{:?}\n", word, synonimous_counter);
    }
}