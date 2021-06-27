mod parsing;
mod utils;
mod actors;
mod messages;

use actix::prelude::*;

use std::sync::Arc;
use std::process;
use std::env;

use crate::actors::{Gatekeeper, PerWordWorker, Worker};
use crate::messages::{SynonymRequest, SynonymsResult};
use parsing::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};
use utils::{Counter, FileReader};

#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS_SECS: u64 = 1;
static MAX_CONCURRENCY: usize = 5;
static MAX_PAGES: i32 = 3;

#[actix_rt::main]
async fn run_actors(words: Vec<String>) {
    let mut words = vec![];

    let w1 = Arc::new("house".to_string());
    let w2 = Arc::new("cat".to_string());
    let w3 = Arc::new("car".to_string());
    let w4 = Arc::new("-1".to_string());

    words.push(w1.clone());
    words.push(w2.clone());
    words.push(w3.clone());
    words.push(w4.clone());

    let worker = Arc::new(SyncArbiter::start(MAX_CONCURRENCY, || Worker));

    let gatekeepers = vec![
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "1".to_string(),
                sleep_time: MIN_TIME_REQUESTS_SECS
            }
            .start(),
        ),
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "2".to_string(),
                sleep_time: MIN_TIME_REQUESTS_SECS
            }
            .start(),
        ),
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "3".to_string(),
                sleep_time: MIN_TIME_REQUESTS_SECS
            }
            .start(),
        ),
    ];

    let gatekeepers = Arc::new(gatekeepers);

    for w in words {
        PerWordWorker {
            target: Arc::new("".to_string()).clone(),
            gatekeepers: gatekeepers.clone(),
            lefting: 3,
            acum: vec![],
        }
        .start()
        .send(SynonymRequest { target: w.clone() })
        .await
        .unwrap();
    }

    println!("stopping system...");
    System::current().stop();
}

fn choose_mode(mode:String, filename: String) {
    let words = FileReader::new(filename).get_words();

    if mode.eq("actors") {
        println!("actors");
        return run_actors(words);
    } else if mode.eq("threads") {
        println!("threads");
        return run_parsers(words);
    } else {

    }
}

fn run_parsers(words: Vec<String>) {
    let p1 = ThesaurusProvider;
    let p2 = YourDictionaryProvider;
    let p3 = MerriamWebsterProvider;

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words_arc = Arc::from(words);

    let controller = Controller::new(words_arc, providers_arc);

    controller.process_words_concurrently();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone())
}
