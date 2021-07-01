mod main_threads;
mod main_actors;

use crate::main_threads::main_threads;
use crate::main_actors::main_actors;

use std::env;
use std::process;
use std::sync::Arc;

#[path = "utils/logger.rs"]
mod logger;
use logger::Logger;

#[path = "utils/file_reader.rs"]
mod file_reader;
use file_reader::FileReader;

const LOG_FILENAME: &str = "log.txt";
const MESSAGE_LOGGER_ERROR: &str = "Unable to open logger file ";
const MESSAGE_OPEN_FILE_ERROR: &str = "Unable to open file";
const MESSAGE_INVALID_MODE: &str = "Invalid mode";


fn usage() -> i32 {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} <actors|threads> <input file>", args[0]);
    println!("Usage: {} <actors|threads> <input file> <max_concurrency> <min_time_request_sec>", args[0]);
    return -1;
}

fn starting(mode: String, threads: usize, timeout: u64) {
    println!(
        "Starting in {} mode, using up to {} threads, and {} secs. as request timeout.\n",
        &mode, &threads, &timeout
    );
}


fn chose_mode(mode: String, filename: String, max_concurrency: usize,
        min_time_request_sec: u64) -> i32 {
    let logger = match Logger::new(LOG_FILENAME) {
        Ok(logger) => Arc::new(logger),
        Err(e) => {
            println!("{} {:?}: {}", MESSAGE_LOGGER_ERROR, LOG_FILENAME, e);
            return -1;
        }
    };

    let f_reader = FileReader::new(filename.clone(), logger.clone());

    let words = match f_reader.get_words() {
        Ok(words) => words,
        Err(e) => {
            println!("{} {:?}: {}", MESSAGE_OPEN_FILE_ERROR, filename, e);
            return -1;
        }
    };

    match mode.as_str() {
        "actors" => {
            main_actors(words, logger.clone(), max_concurrency, min_time_request_sec);
            return 0;
        }
        "threads" => {
            starting(mode, max_concurrency, min_time_request_sec);
            main_threads(words, logger.clone(), max_concurrency, min_time_request_sec);
            return 0;
        }
        _ => {
            println!("{}: {}", MESSAGE_INVALID_MODE, mode);
            return 0;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 6 {
        process::exit(usage());
    }
    let mut max_concurrency = 1;
    let mut min_time_request_sec = 0;

    if args.len() == 5 {
         max_concurrency = match args[3].parse::<usize>() {
            Ok(result) => result,
            Err(_) => 0
        };

        min_time_request_sec = match args[4].parse::<u64>() {
            Ok(result) => result,
            Err(_) => 0
        };
    }
    process::exit(chose_mode(args[1].clone(), args[2].clone(),
        max_concurrency, min_time_request_sec));
}
