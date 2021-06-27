use actix::prelude::*;
#[path = "utils/counter.rs"]
mod counter;
use counter::Counter;
use actix::{Actor, Context, SyncContext};
use std::sync::{Arc};
use std::vec;


use crate::logger::Logger;
use crate::messages::*;
use crate::parsing::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};


pub struct Worker;

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<WorkerSynonymsRequest> for Worker {
    type Result = ();

    fn handle(
        &mut self,
        request: WorkerSynonymsRequest,
        _ctx: &mut SyncContext<Self>,
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

        let syn = match request.parser_key.as_str() {
            "1" => parsers[0].clone().parse(tmp.clone()),
            "2" => parsers[1].clone().parse(tmp.clone()),
            "3" => parsers[2].clone().parse(tmp.clone()),
            _ => vec![],
        };

        match request.response_addr.try_send(SynonymsResult {
            synonyms: Arc::new(syn),
        }) {
            Ok(_) => {}
            Err(_) => panic!("Error al enviar resultados de sinonimos"),
        }
    }
}

pub struct Gatekeeper {
    pub worker: Arc<Addr<Worker>>,
    pub last: std::time::Instant,
    pub parser_key: String,
    pub sleep_time: u64,
    pub logger: Arc<Logger>,
}

impl Actor for Gatekeeper {
    type Context = Context<Self>;
}

impl Handler<GatekeeperRequest> for Gatekeeper {
    type Result = ();

    fn handle(&mut self, msg: GatekeeperRequest, _ctx: &mut Context<Self>) -> Self::Result {
        self.logger.write(format!("INFO: [T] handling {:?}\n", msg.target.clone()));

        let elapsed = std::time::Instant::now()
            .duration_since(self.last)
            .as_secs();
        if elapsed < self.sleep_time {
            self.logger.write(format!("INFO: [T] Sleeping by {:?} secs.\n", (self.sleep_time - elapsed)));
            std::thread::sleep(std::time::Duration::from_secs(self.sleep_time - elapsed));
            self.logger.write(format!("INFO: [T] Awaking\n"));
        }

        self.logger.write(format!("INFO: [T] Making request for {:?}\n", msg.target.clone()));

        let worker_request = WorkerSynonymsRequest {
            response_addr: msg.response_addr.clone(),
            target: msg.target.clone(),
            parser_key: self.parser_key.clone(),
            logger: self.logger.clone(),
        };

        match self.worker.try_send(worker_request) {
            Ok(_) => {
                self.last = std::time::Instant::now();
            }
            Err(_) => {
                panic!("No se pudo enviar request a los workers!");
            }
        }
    }
}

pub struct PerWordWorker {
    pub target: Arc<String>,
    pub gatekeepers: Arc<Vec<Arc<Addr<Gatekeeper>>>>,
    pub acum: Vec<String>,
    pub lefting: u32,
    pub logger: Arc<Logger>,
    pub logger_result: Arc<Logger>
}

impl Actor for PerWordWorker {
    type Context = Context<Self>;
}

impl Handler<SynonymRequest> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, request: SynonymRequest, ctx: &mut Context<Self>) -> Self::Result {
        self.logger.write(format!("INFO: Asking synonym for {:?}\n", request.target));
        let me = Arc::new(ctx.address());
        self.target = request.target.clone();

        for gatekeeper in self.gatekeepers.iter() {
            let gatekeeper_request = GatekeeperRequest {
                response_addr: me.clone(),
                target: self.target.clone(),
            };

            match gatekeeper.try_send(gatekeeper_request) {
                Ok(_result) => {
                    self.logger.write(format!("INFO: Sended to [T]\n"));
                }
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
        self.logger.write(format!("INFO: *** sinonimos para {:?} recibidos\n", self.target));
        let mut tmp = self.lefting;
        tmp -= 1;
        self.acum.extend_from_slice(&result.synonyms.clone());
        self.lefting = tmp;
        if tmp == 0 {
            self.logger.write(format!("INFO: Palabra: {:?} tiene sinónimos:\n", self.target));
            let tmp: String = (*self.target).clone();
            let tmp2 = self.acum.clone();
        

            Counter::count(
                tmp, 
                tmp2,
                self.logger_result.clone()
            );
        }
    }
}
