use std::sync::Arc;
use std::collections::HashMap;

use crate::Logger;

/* COUNTER */

pub struct Counter;

impl Counter {
    pub fn count(word: String, synonyms: Vec<String>, logger: Arc<Logger>) {
        let mut s_counter = HashMap::new();

        for s in synonyms {
            let entry = s_counter.entry(s).or_insert(0);
            *entry += 1;
        }

        println!("------- [ Synonyms for: `{}` ] -------", word);

        let data = s_counter
            .iter()
            .map(|(k, v)| {
                if *v == 1 {
                    format!("{}", k)
                } else {
                    format!("{} ({})", k, v)
                }
            })
            .collect::<Vec<String>>()
            .join(" | ");

        println!("{}\n", data);

        logger.info(format!("Mostrando resultados para {:?} en pantalla", word));
    }
}
