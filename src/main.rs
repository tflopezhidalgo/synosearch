mod actors;
mod messages;
mod parsing;

use actix::prelude::*;

use std::env;
use std::fmt::Debug;
use std::process;
use std::sync::Arc;

use crate::actors::CounterActor;
use crate::actors::{Gatekeeper, PerWordWorker, Worker};
use crate::messages::SynonymRequest;
use parsing::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};

#[path = "threading/controller.rs"]
mod controller;
use controller::Controller;

#[path = "utils/logger.rs"]
mod logger;
use logger::Logger;

#[path = "utils/file_reader.rs"]
mod file_reader;
use file_reader::FileReader;

const NOTIFY_FRECUENCY: u64 = 1;
const MIN_TIME_REQUESTS_SECS: u64 = 10;
const MAX_CONCURRENCY: usize = 10;
const MAX_PAGES: i32 = 3;
const LOG_FILENAME: &str = "log.txt";

#[derive(Debug)]
pub enum AvailableParsers{
    YourDictionary,
    MerriamWebster,
    Thesaurus
}

fn usage() -> i32 {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} <actors|threads> <input file>", args[0]);
    return -1;
}

fn starting(mode: String, threads: usize, timeout: u64, logfile: String, inputfile: String) {
    println!();
    println!(
        "Taking words from {:?}. Logging messages will be saved to: {:?}.",
        &inputfile, &logfile
    );
    println!(
        "Starting in {} mode, using up to {} threads, and {} secs. as request timeout.",
        &mode, &threads, &timeout
    );
    println!();
}

fn run_actors(words: Vec<String>, logger: Arc<Logger>) {
    let system = System::new();
    let mut words_arc = vec![];
    for w in words {
        words_arc.push(Arc::new(w));
    }

    let worker = Arc::new(SyncArbiter::start(MAX_CONCURRENCY, || Worker));

    let parsers = vec!(
        AvailableParsers::YourDictionary,
        AvailableParsers::MerriamWebster,
        AvailableParsers::Thesaurus
    );

    system.block_on(async {
        let mut gatekeepers = vec![];
        for parser_type in parsers {
            gatekeepers.push(Arc::new(
                Gatekeeper {
                    worker: worker.clone(),
                    last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                    parser: Arc::new(parser_type),
                    sleep_time: MIN_TIME_REQUESTS_SECS,
                    logger: logger.clone(),
                }
                .start(),
            ))
        }

        let gatekeepers = Arc::new(gatekeepers);

        let c_actor = Arc::new(
            CounterActor {
                limit: words_arc.len() as u32,
                count: 0,
            }
            .start(),
        );

        let mut word_workers = vec![];

        for w in words_arc {
            word_workers.push(
                PerWordWorker {
                    target: w.clone(),
                    gatekeepers: gatekeepers.clone(),
                    lefting: gatekeepers.len() as u32,
                    acum: vec![],
                    logger: logger.clone(),
                    counter: c_actor.clone(),
                }
                .start()
                .send(SynonymRequest { target: w.clone() })
                .await,
            );
        }

        let _ = word_workers
            .iter()
            .map(|future| match future {
                Ok(()) => (),
                Err(e) => {
                    println!("Unable to send word to actor: {}", e)
                }
            })
            .collect::<()>();
        ()
    });

    match system.run() {
        Ok(_) => {}
        Err(e) => panic!("Unable to run actors' system: {}", e),
    };
}

fn run_threads(words: Vec<String>, logger: Arc<Logger>) {
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

fn chose_mode(mode: String, filename: String) -> i32 {
    let logger = match Logger::new(LOG_FILENAME) {
        Ok(logger) => Arc::new(logger),
        Err(e) => {
            println!("Unable to open logger file {:?}: {}", LOG_FILENAME, e);
            return -1;
        }
    };

    let f_reader = FileReader::new(filename.clone(), logger.clone());

    let words = match f_reader.get_words() {
        Ok(words) => words,
        Err(e) => {
            println!("Unable to open file {:?}: {}", filename, e);
            return -1;
        }
    };

    match mode.as_str() {
        "actors" => {
            starting(
                mode,
                MAX_CONCURRENCY,
                MIN_TIME_REQUESTS_SECS,
                LOG_FILENAME.to_string(),
                filename,
            );
            run_actors(words, logger.clone());
            return 0;
        }
        "threads" => {
            starting(
                mode,
                MAX_CONCURRENCY,
                MIN_TIME_REQUESTS_SECS,
                LOG_FILENAME.to_string(),
                filename,
            );
            run_threads(words, logger.clone());
            return 0;
        }
        _ => {
            println!("Invalid mode: {}", mode);
            return 0;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(usage());
    }
    process::exit(chose_mode(args[1].clone(), args[2].clone()));
}
