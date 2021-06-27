use std::sync::Arc;
use actix::prelude::*;
use actix::{Actor, Context, SyncContext};

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

        if request.parser_key == "1" {
            let parser = &ThesaurusProvider::new(request.logger.clone());
            let syn = Arc::new(parser.parse(tmp.clone()));

            request
                .response_addr
                .try_send(SynonymsResult { synonyms: syn })
                .unwrap();
        } else if request.parser_key == "2" {
            let parser = &YourDictionaryProvider::new(request.logger.clone());
            let syn = Arc::new(parser.parse(tmp.clone()));

            request
                .response_addr
                .try_send(SynonymsResult { synonyms: syn })
                .unwrap();
        } else {
            let parser = &MerriamWebsterProvider::new(request.logger.clone());
            let syn = Arc::new(parser.parse(tmp.clone()));

            request
                .response_addr
                .try_send(SynonymsResult { synonyms: syn })
                .unwrap();
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
        println!("[T] handling {:?}", msg.target.clone());

        let elapsed = std::time::Instant::now()
            .duration_since(self.last)
            .as_secs();
        if elapsed < self.sleep_time {
            println!(
                "[T] Sleeping by {:?} secs.",
                (self.sleep_time - elapsed)
            );
            std::thread::sleep(std::time::Duration::from_secs(
                self.sleep_time - elapsed,
            ));
            println!("[T] Awaking");
        }

        println!("[T] Making request for {:?}", msg.target.clone());

        self.worker
            .try_send(WorkerSynonymsRequest {
                response_addr: msg.response_addr.clone(),
                target: msg.target.clone(),
                parser_key: self.parser_key.clone(),
                logger: self.logger.clone(),
            })
            .unwrap();

        self.last = std::time::Instant::now();
    }
}

pub struct PerWordWorker {
    pub target: Arc<String>,
    pub gatekeepers: Arc<Vec<Arc<Addr<Gatekeeper>>>>,
    pub acum: Vec<String>,
    pub lefting: u32,
}

impl Actor for PerWordWorker {
    type Context = Context<Self>;
}

impl Handler<SynonymRequest> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, request: SynonymRequest, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Asking synonym for {:?}", request.target);
        let me = Arc::new(_ctx.address());
        self.target = request.target.clone();

        for gk in self.gatekeepers.iter() {
            gk.try_send(GatekeeperRequest {
                response_addr: me.clone(),
                target: self.target.clone(),
            })
            .unwrap();
            println!("Sended to [T]");
        }
    }
}

impl Handler<SynonymsResult> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, msg: SynonymsResult, _ctx: &mut Context<Self>) -> Self::Result {
        println!("*** sinonimos para {:?} recibidos", self.target);
        let mut tmp = self.lefting;
        tmp -= 1;
        self.acum.extend_from_slice(&msg.synonyms.clone());
        self.lefting = tmp;
        if tmp == 0 {
            println!("Palabra: {:?} tiene sinónimos:", self.target);
            println!("{:?}", self.acum.join(", "));
        }
    }
}
