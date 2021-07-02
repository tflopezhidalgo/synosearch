use crate::Logger;

use actix::prelude::{Actor, System, SyncArbiter};
use std::sync::Arc;
#[path = "./actors/mod.rs"]
mod actors;

use actors::counter_actor::CounterActor;
use actors::perwordworker::PerWordWorker;
use actors::worker::Worker;
use actors::gatekeeper::Gatekeeper;
use actors::messages::SynonymRequest;

const MESSAGE_WORKER_ERROR: &str = "Unable to send word to actor:";
const MESSAGE_SYSTEM_ERROR: &str = "Unable to run actors' system: ";

#[derive(Debug)]
pub enum AvailableParsers{
    YourDictionary,
    MerriamWebster,
    Thesaurus
}

pub fn main_actors(words: Vec<String>, logger: Arc<Logger>, max_concurrency: usize,
        min_time_request_sec: u64) {
    let system = System::new();
    let mut words_arc = vec![];
    for w in words {
        words_arc.push(Arc::new(w));
    }

    let worker = Arc::new(SyncArbiter::start(max_concurrency, || Worker));

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
                    sleep_time: min_time_request_sec,
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
                    println!("{} {}", MESSAGE_WORKER_ERROR, e)
                }
            })
            .collect::<()>();
        ()
    });

    match system.run() {
        Ok(_) => {}
        Err(e) => panic!("{} {}", MESSAGE_SYSTEM_ERROR, e),
    };
}
