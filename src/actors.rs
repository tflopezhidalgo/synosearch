#[path = "utils/counter.rs"]
mod counter;
use counter::Counter;

use actix::prelude::*;
use actix::{Actor, Context, SyncContext};

use std::sync::Arc;
use std::vec;

use crate::logger::Logger;
use crate::messages::*;
use crate::parsing::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};

/// Worker actor. Used in a pool of actors.
/// Responsible for requesting to the synonyms page
pub struct Worker;

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<WorkerSynonymsRequest> for Worker {
    type Result = ();

    fn handle(
        &mut self,
        request: WorkerSynonymsRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let tmp = (*request.target).clone();

        let parsers: Vec<Box<Arc<dyn Parser>>> = vec![
            Box::new(Arc::new(ThesaurusProvider::new(request.logger.clone()))),
            Box::new(Arc::new(YourDictionaryProvider::new(
                request.logger.clone(),
            ))),
            Box::new(Arc::new(MerriamWebsterProvider::new(
                request.logger.clone(),
            ))),
        ];

        let syn = match request.parser_i {
            0 => parsers[0].clone().parse(tmp.clone()),
            1 => parsers[1].clone().parse(tmp.clone()),
            2 => parsers[2].clone().parse(tmp.clone()),
            _ => vec![],
        };

        request.logger.info(format!("Worker making request for {}", tmp));
        match request.response_addr.try_send(SynonymsResult {
            synonyms: Arc::new(syn),
        }) {
            Ok(_) => {}
            Err(_) => panic!("Error al enviar resultados de sinonimos"),
        }
    }
}

/// Responsible for keep the consecutives request to the same site 
/// rate limited
pub struct Gatekeeper {
    /// Worker's pool reference for requesting synonyms.
    pub worker: Arc<Addr<Worker>>,

    /// Instant where the last request was made.
    pub last: std::time::Instant,

    /// Parser index.
    pub parser_i: u32,

    /// Sleep time between concurrent request to the same site.
    pub sleep_time: u64,

    /// Logger reference.
    pub logger: Arc<Logger>,
}

impl Actor for Gatekeeper {
    type Context = Context<Self>;
}

impl Handler<GatekeeperRequest> for Gatekeeper {
    type Result = ();

    fn handle(&mut self, msg: GatekeeperRequest, _: &mut Context<Self>) -> Self::Result {
        self.logger.info(format!("Gatekeeper recieved request for word {}", msg.target));
        let elapsed = std::time::Instant::now()
            .duration_since(self.last)
            .as_secs();
        if elapsed < self.sleep_time {
            let sleep_time = std::time::Duration::from_secs(self.sleep_time - elapsed);
            self.logger.info(format!("Gatekeeper sleeping by {} secs.", sleep_time.as_secs()));
            std::thread::sleep(sleep_time);
        }

        let worker_request = WorkerSynonymsRequest {
            response_addr: msg.response_addr,
            target: msg.target.clone(),
            parser_i: self.parser_i,
            logger: self.logger.clone(),
        };

        match self.worker.try_send(worker_request) {
            Ok(_) => {
                self.logger.info(
                    format!(
                        "Gatekeeper sended request to worker for word {}.", msg.target.clone()
                    )
                );
                self.last = std::time::Instant::now();
            }
            Err(_) => {
                panic!("No se pudo enviar request a los workers!");
            }
        }
    }
}

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

/// Actor responsible for carry the count of current
/// finished words. When the count (all the words) has been reached
/// this actor will stop the entire system.
pub struct CounterActor {
    /// Limit to reach the count.
    pub limit: u32,

    /// Actual count of processed words.
    pub count: u32,
}

impl Actor for CounterActor {
    type Context = Context<Self>;
}

impl Handler<Increment> for CounterActor {
    type Result = ();

    fn handle(&mut self, _: Increment, _: &mut Context<Self>) -> Self::Result {
        self.count += 1;
        if self.count == self.limit {
            System::current().stop();
        }
    }
}
