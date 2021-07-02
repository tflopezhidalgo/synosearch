#[path = "../utils/counter.rs"]
mod counter;

#[path = "../parsing/parser.rs"]
mod parser;

use super::messages::{SynonymRequest, SynonymsResult, GatekeeperRequest, Increment};
use super::gatekeeper::Gatekeeper;
use super::counter_actor::CounterActor;

use actix::prelude::*;
use actix::{Actor, Context};

use std::sync::Arc;

use crate::logger::Logger;

use counter::Counter;

/// Worker that keeps track of one word. This allows the worker to know
/// when the word's synonyms has been requested and arrived. 
pub struct PerWordWorker {

    /// Target word for synonyms request.
    pub target: Arc<String>,

    /// Available gatekeepers references.
    pub gatekeepers: Arc<Vec<Arc<Addr<Gatekeeper>>>>,

    /// Gatekeepers count, this allows the worker to stop when 
    /// all gatekeepers have been messaged.
    pub counter: Arc<Addr<CounterActor>>,

    /// Acumulated synonyms.
    pub acum: Vec<String>,

    /// Lefting synonyms count.
    pub lefting: u32,

    /// Logger reference.
    pub logger: Arc<Logger>,
}

impl Actor for PerWordWorker {
    type Context = Context<Self>;
}

impl Handler<SynonymRequest> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, request: SynonymRequest, ctx: &mut Context<Self>) -> Self::Result {
        let me = Arc::new(ctx.address().recipient());

        self.logger.info(
            format!("Sending synonym request for {} word to gatekeeper", self.target)
        );
        for gatekeeper in self.gatekeepers.iter() {
            let gatekeeper_request = GatekeeperRequest {
                response_addr: me.clone(),
                target: request.target.clone(),
            };

            match gatekeeper.try_send(gatekeeper_request) {
                Ok(_result) => {}
                Err(_e) => {
                    panic!("No se pudo enviar el mensaje al gatekeeper");
                }
            };
        }
    }
}

impl Handler<SynonymsResult> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, result: SynonymsResult, _: &mut Context<Self>) -> Self::Result {
        let mut tmp = self.lefting;
        tmp -= 1;
        self.acum.extend_from_slice(&result.synonyms.clone());
        self.lefting = tmp;
        if tmp == 0 {
            let tmp: String = (*self.target).clone();
            let tmp2 = self.acum.clone();
            Counter::count(tmp, tmp2, self.logger.clone());
            self.counter.do_send(Increment);
        }
    }
}