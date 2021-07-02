#[path = "../utils/counter.rs"]
mod counter;

#[path = "../parsing/parser.rs"]
mod parser;

use super::worker::Worker;
use super::messages::{WorkerSynonymsRequest, GatekeeperRequest};

use actix::fut::wrap_future;
use actix::prelude::*;
use actix::{Actor, Context};
use actix::clock::sleep;

use std::time::{Duration, Instant};
use std::sync::Arc;
use std::fmt::Display;

use crate::main_actors::AvailableParsers;
use crate::logger::Logger;

/// Responsible for keep the consecutives request to the same site 
/// rate limited
pub struct Gatekeeper {
    /// Worker's pool reference for requesting synonyms.
    pub worker: Arc<Addr<Worker>>,

    /// Instant where the last request was made.
    pub last: Instant,

    /// Parser index.
    pub parser: Arc<AvailableParsers>,

    /// Sleep time between concurrent request to the same site.
    pub sleep_time: u64,

    /// Logger reference.
    pub logger: Arc<Logger>,
}

impl Actor for Gatekeeper {
    type Context = Context<Self>;
}

impl Display for Gatekeeper {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gatekeeper")
    }
}

impl Handler<GatekeeperRequest> for Gatekeeper {
    type Result = ();

    fn handle(&mut self, msg: GatekeeperRequest, ctx: &mut Context<Self>) -> Self::Result {
        println!("[{}][{:?}] Recibió palabra: {}", self, self.parser, msg.target);

        let worker_request = WorkerSynonymsRequest {
            target: msg.target.clone(),
            response_addr: msg.response_addr,
            parser: self.parser.clone(),
            logger: self.logger.clone(),
        };
        match self.worker.try_send(worker_request) {
            Ok(_) => {
                println!("[Gatekeeper][{:?}] Transmitiendo palabra: {}", self.parser, msg.target);
                self.logger.info(
                    format!(
                        "Gatekeeper sended request to worker for word {}.", msg.target.clone()
                    )
                );
            }
            Err(_) => {
                panic!("No se pudo enviar request a los workers!");
            }
        }
        match *self.parser {
            AvailableParsers::MerriamWebster => {
                let sleep_time = Duration::from_secs(10);
                ctx.wait(wrap_future(sleep(sleep_time)));
            },
            _ => { }
        }

        println!("[Gatekeeper][{:?}] Finalizando, palabra: {}", self.parser, msg.target);
    }
}