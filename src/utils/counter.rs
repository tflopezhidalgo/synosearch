use std::collections::HashMap;
use std::sync::Arc;

use crate::Logger;

/* COUNTER */

/// Counter struct. Responsible for format the 
/// synonyms results in the screen with a nice
/// syntax. 
pub struct Counter;

impl Counter {
    /// Formatting method. Accepts a target word (for whose the synonyms is)
    /// and a list of synonyms to be formatted. Also received the logger
    /// referecne to log when the synonyms are being showed in the screen.
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

        logger.info(format!("[Counter] Showing results for {:?}", word));
    }
}
