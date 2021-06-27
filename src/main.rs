mod parsing;
mod actors;
mod messages;

use actix::prelude::*;

use std::sync::Arc;
use std::process;
use std::env;

use crate::actors::{Gatekeeper, PerWordWorker, Worker};
use crate::messages::{SynonymRequest};
use parsing::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};

#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

#[path = "utils/logger.rs"] mod logger;
use logger::Logger;

#[path = "utils/file_reader.rs"] mod file_reader;
use file_reader::FileReader;

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS_SECS: u64 = 1;
static MAX_CONCURRENCY: usize = 5;
static MAX_PAGES: i32 = 3;
const FILENAME: &str = "src/log.txt";

#[actix_rt::main]
async fn run_actors(words: Vec<String>, logger: Arc<Logger>) {
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
                sleep_time: MIN_TIME_REQUESTS_SECS,
                logger: logger.clone()
            }
            .start(),
        ),
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "2".to_string(),
                sleep_time: MIN_TIME_REQUESTS_SECS,
                logger: logger.clone()
            }
            .start(),
        ),
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "3".to_string(),
                sleep_time: MIN_TIME_REQUESTS_SECS,
                logger: logger.clone()
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

fn choose_mode(mode: String, filename: String) {
    let logger = Arc::from(Logger::new(FILENAME));

    let words = FileReader::new(filename, logger.clone()).get_words();

    if mode.eq("actors") {
        println!("actors");
        return run_actors(words, logger.clone());
    } else if mode.eq("threads") {
        println!("Run mode threads");
        logger.write("INFO: Run program mod threads\n".to_string());
        run_parsers(words, logger.clone());
    } else {
        println!("Unknown mode\n");
    }
}

fn run_parsers(words: Vec<String>, logger: Arc<Logger>) {
    let p1 = ThesaurusProvider::new(logger.clone());
    let p2 = YourDictionaryProvider::new(logger.clone());
    let p3 = MerriamWebsterProvider::new(logger.clone());

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words_arc = Arc::from(words);

    let controller = Controller::new(words_arc, providers_arc, logger);

    controller.process_words_concurrently();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone())
}
