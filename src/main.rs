mod parsing;

use actix::prelude::*;
use actix::{Actor, Context, SyncContext};

use std::sync::Arc;
use std::process;
use std::env;

use parsing::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};

mod utils;
use utils::{
    Counter,
    FileReader,
    Logger
};

#[path = "threading/controller.rs"] mod controller;
use controller::Controller;

static NOTIFY_FRECUENCY: u64 = 1;
static MIN_TIME_REQUESTS_SECS: u64 = 1;
static MAX_CONCURRENCY: usize = 5;
static MAX_PAGES: i32 = 3;


/* Messages */

#[derive(Message)]
#[rtype(result = "()")]
struct SynonymRequest {
    target: Arc<String>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct GatekeeperRequest {
    target: Arc<String>,
    response_addr: Arc<Addr<PerWordWorker>>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct WorkerSynonymsRequest {
    target: Arc<String>,
    response_addr: Arc<Addr<PerWordWorker>>,
    parser_key: String,
}

#[derive(Message)]
#[rtype(result = "()")]
struct SynonymsResult {
    synonyms: Arc<Vec<String>>,
}

/* Actors */

struct Worker;

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
            let parser = &ThesaurusProvider;
            let syn = Arc::new(parser.parse(tmp.clone()));

            request
                .response_addr
                .try_send(SynonymsResult { synonyms: syn })
                .unwrap();
        } else if request.parser_key == "2" {
            let parser = &YourDictionaryProvider;
            let syn = Arc::new(parser.parse(tmp.clone()));

            request
                .response_addr
                .try_send(SynonymsResult { synonyms: syn })
                .unwrap();
        } else {
            let parser = &MerriamWebsterProvider;
            let syn = Arc::new(parser.parse(tmp.clone()));

            request
                .response_addr
                .try_send(SynonymsResult { synonyms: syn })
                .unwrap();
        }
    }
}

struct Gatekeeper {
    worker: Arc<Addr<Worker>>,
    last: std::time::Instant,
    parser_key: String,
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
        if elapsed < MIN_TIME_REQUESTS_SECS {
            println!(
                "[T] Sleeping by {:?} secs.",
                (MIN_TIME_REQUESTS_SECS - elapsed)
            );
            std::thread::sleep(std::time::Duration::from_secs(
                MIN_TIME_REQUESTS_SECS - elapsed,
            ));
            println!("[T] Awaking");
        }

        println!("[T] Making request for {:?}", msg.target.clone());

        self.worker
            .try_send(WorkerSynonymsRequest {
                response_addr: msg.response_addr.clone(),
                target: msg.target.clone(),
                parser_key: self.parser_key.clone(),
            })
            .unwrap();

        self.last = std::time::Instant::now();
    }
}

struct PerWordWorker {
    target: Arc<String>,
    gatekeepers: Arc<Vec<Arc<Addr<Gatekeeper>>>>,
    acum: Vec<String>,
    lefting: u32,
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

#[actix_rt::main]
async fn run_actors(words: Vec<String>) {
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
            }
            .start(),
        ),
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "2".to_string(),
            }
            .start(),
        ),
        Arc::new(
            Gatekeeper {
                worker: worker.clone(),
                last: std::time::Instant::now() - std::time::Duration::from_secs(10000),
                parser_key: "3".to_string(),
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

fn choose_mode(mode:String, filename: String) {
    let words = FileReader::new(filename).get_words();

    if mode.eq("actors") {
        println!("actors");
        return run_actors(words);
    } else if mode.eq("threads") {
        println!("threads");
        return run_parsers(words);
    } else {

    }
}

fn run_parsers(words: Vec<String>) {
    let p1 = ThesaurusProvider;
    let p2 = YourDictionaryProvider;
    let p3 = MerriamWebsterProvider;

    let mut providers: Vec<Box<dyn Parser + Send + Sync>> = Vec::new();
    providers.push(Box::new(p1));
    providers.push(Box::new(p2));
    providers.push(Box::new(p3));

    let providers_arc = Arc::from(providers);

    let words_arc = Arc::from(words);

    let controller = Controller::new(words_arc, providers_arc);

    controller.process_words_concurrently();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone())
}