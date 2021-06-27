use crate::Arc;
use crate::Logger;
use std::collections::HashMap;

/* COUNTER */

pub struct Counter;

impl Counter {
    pub fn count(word: String, synonimous: Vec<String>, logger: Arc<Logger>) {
        let mut synonimous_counter = HashMap::new();
        for s in synonimous {
            let entry = synonimous_counter.entry(s).or_insert(0);
            *entry += 1;
        }

        
        logger.write(format!(
            "\nWORD: {}\nSINONIMOUS: \n",
            word
        ));

        for c in &synonimous_counter {
            let key = c.0;
            let value = c.1;
            if *value == 1 {
                logger.write(format!("{}\n", key));
            } else {
                logger.write(format!("{} ({})\n", key, value));
            }
        }
        
        logger.write(format!("\n\n"));
        println!("WORD: {}\nSINONIMOUS: \n{:?}\n", word, synonimous_counter);
    }
}
