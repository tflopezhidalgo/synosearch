use std::collections::HashMap;

pub struct Counter {}

impl Counter {
    pub fn count(synonimous: Vec<String>) { 
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
        println!("{:?}", synonimous_counter);
    }
}