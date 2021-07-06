use super::messages::{GatekeeperRequest, WorkerSynonymsRequest};
use super::worker::Worker;

use actix::clock::sleep;
use actix::fut::wrap_future;
use actix::prelude::*;
use actix::{Actor, Context};

use std::fmt::Display;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::logger::Logger;
use crate::main_actors::AvailableParsers;

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
        self.logger.info(format!(
            "[{}][{:?}] Received GatekeeperRequest for: {}",
            self, self.parser, msg.target
        ));

        let worker_request = WorkerSynonymsRequest {
            target: msg.target.clone(),
            response_addr: msg.response_addr,
            parser: self.parser.clone(),
            logger: self.logger.clone(),
        };
        match self.worker.try_send(worker_request) {
            Ok(_) => {
                self.logger.info(format!(
                    "[{}][{:?}] Sended worker request for word {}.",
                    self,
                    self.parser,
                    msg.target.clone()
                ));
            }
            Err(_) => {
                panic!("No se pudo enviar request a los workers!");
            }
        }

        let sleep_time = Duration::from_secs(self.sleep_time);
        ctx.wait(wrap_future(sleep(sleep_time)));
    }
}
