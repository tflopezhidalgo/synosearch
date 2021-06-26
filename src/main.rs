mod parsing;

use std::sync::Arc;
use actix::prelude::*;
use actix::{Actor, Context, System, SyncContext};

use parsing::{
    ThesaurusProvider,
    MarianWebsterProvider,
    YourDictionaryProvider,
    Parser
};

static MIN_TIME_REQUESTS_SECS: u64 = 1;
static MAX_CONCURRENCY: usize = 1;

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
    response_addr: Arc<Addr<PerWordWorker>>
}

#[derive(Message)]
#[rtype(result = "()")]
struct WorkerSynonymsRequest {
    _type: String,
    target: Arc<String>,
    response_addr: Arc<Addr<PerWordWorker>>
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

    fn handle(&mut self, request: WorkerSynonymsRequest, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let tmp = (*request.target).clone();

        if request._type == "marian" {
            let syn = Arc::new(
                MarianWebsterProvider{ 
                    url: "".to_string() 
                }
                .parse(tmp.clone())
            );

            request.response_addr.try_send(SynonymsResult{ synonyms: syn }).unwrap()

        } else if request._type == "thesaurus" {
            let syn = Arc::new(
                ThesaurusProvider{ 
                    url: "".to_string() 
                }
                .parse(tmp.clone())
            );

            request.response_addr.try_send(SynonymsResult{ synonyms: syn }).unwrap()

        } else {
            let syn = Arc::new(
                YourDictionaryProvider{ 
                    url: "".to_string() 
                }
                .parse(tmp.clone())
            );

            request.response_addr.try_send(SynonymsResult{ synonyms: syn }).unwrap()
        }
    }
}

struct TWordGateKeeper { 
    worker: Arc<Addr<Worker>>,
    last: std::time::Instant,
}

impl Actor for TWordGateKeeper {
    type Context = Context<Self>;
}

impl Handler<GatekeeperRequest> for TWordGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: GatekeeperRequest, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[T] handling {:?}", msg.target.clone());

        let elapsed = std::time::Instant::now().duration_since(self.last).as_secs();
        if  elapsed < MIN_TIME_REQUESTS_SECS {
            println!("[T] Sleeping by {:?} secs.", (MIN_TIME_REQUESTS_SECS - elapsed));
            std::thread::sleep(std::time::Duration::from_secs(MIN_TIME_REQUESTS_SECS - elapsed));
            println!("[T] Awaking");
        }

        println!("[T] Making request for {:?}", msg.target.clone());

        self.worker.try_send(
            WorkerSynonymsRequest{ 
                response_addr: msg.response_addr.clone(), 
                target: msg.target.clone(),
                _type: "thesaurus".to_string(),
            }
        )
        .unwrap();

        self.last = std::time::Instant::now();
    }
}

struct YourDictionaryGateKeeper {
    worker: Arc<Addr<Worker>>,
    last: std::time::Instant,
}

impl Actor for YourDictionaryGateKeeper {
    type Context = Context<Self>;
}

impl Handler<GatekeeperRequest> for YourDictionaryGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: GatekeeperRequest, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[Y] handling {:?}", msg.target.clone());

        let elapsed = std::time::Instant::now().duration_since(self.last).as_secs();
        if  elapsed < MIN_TIME_REQUESTS_SECS {
            println!("[Y] Sleeping by {:?} secs.", (MIN_TIME_REQUESTS_SECS - elapsed));
            std::thread::sleep(std::time::Duration::from_secs(MIN_TIME_REQUESTS_SECS - elapsed));
            println!("[Y] Awaking");
        }

        println!("[Y] Making request for {:?}", msg.target.clone());
        self.worker.try_send(
            WorkerSynonymsRequest{ 
                response_addr: msg.response_addr.clone(), 
                target: msg.target.clone(),
                _type: "yourdictionary".to_string(),
            }
        )
        .unwrap();
        self.last = std::time::Instant::now();
    }
}

struct MarianWebGateKeeper { 
    worker: Arc<Addr<Worker>>,
    last: std::time::Instant,
}

impl Actor for MarianWebGateKeeper {
    type Context = Context<Self>;
}

impl Handler<GatekeeperRequest> for MarianWebGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: GatekeeperRequest, _ctx: &mut Context<Self>) -> Self::Result {
        println!("[M] handling {:?}", msg.target.clone());

        let elapsed = std::time::Instant::now().duration_since(self.last).as_secs();
        if  elapsed < MIN_TIME_REQUESTS_SECS {
            println!("[M] Sleeping by {:?} secs.", (MIN_TIME_REQUESTS_SECS - elapsed));
            std::thread::sleep(std::time::Duration::from_secs(MIN_TIME_REQUESTS_SECS - elapsed));
            println!("[M] Awaking");
        }

        println!("[M] Making request for {:?}", msg.target.clone());

        self.worker.try_send(
            WorkerSynonymsRequest{ 
                response_addr: msg.response_addr.clone(), 
                target: msg.target.clone(),
                _type: "marian".to_string(),
            }
        )
        .unwrap();

        self.last = std::time::Instant::now();
    }
}
struct PerWordWorker {
    target: Arc<String>,
    t_gate_keeper: Arc<Addr<TWordGateKeeper>>,
    y_gate_keeper: Arc<Addr<YourDictionaryGateKeeper>>,
    m_gate_keeper: Arc<Addr<MarianWebGateKeeper>>,
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

        self.t_gate_keeper.try_send(GatekeeperRequest{ response_addr: me.clone(), target: self.target.clone() }).unwrap();
        println!("Sended to [T]");

        self.y_gate_keeper.try_send(GatekeeperRequest{ response_addr: me.clone(), target: self.target.clone() }).unwrap();
        println!("Sended to [Y]");

        self.m_gate_keeper.try_send(GatekeeperRequest{ response_addr: me.clone(), target: self.target.clone() }).unwrap();
        println!("Sended to [M]");
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
async fn main() {
    let mut words = vec!();

    let w1 = Arc::new("house".to_string());
    let w2 = Arc::new("cat".to_string());
    let w3 = Arc::new("car".to_string());
    let w4 = Arc::new("-1".to_string());

    words.push(w1.clone());
    words.push(w2.clone());
    words.push(w3.clone());
    words.push(w4.clone());

    let worker = Arc::new(SyncArbiter::start(MAX_CONCURRENCY, || Worker));

    let m_gk = Arc::new(
        MarianWebGateKeeper{
            worker: worker.clone(),
            last: std::time::Instant::now() - std::time::Duration::from_secs(10000)
        }.start());

    let t_gk = Arc::new(
        TWordGateKeeper{
            worker: worker.clone(),
            last: std::time::Instant::now() - std::time::Duration::from_secs(10000)
        }.start());


    let y_gk = Arc::new(
        YourDictionaryGateKeeper{ 
            worker: worker.clone(),
            last: std::time::Instant::now() - std::time::Duration::from_secs(10000)
        }.start());

    for w in words {
        PerWordWorker { 
            target: Arc::new("".to_string()).clone(), 
            t_gate_keeper: t_gk.clone(),
            m_gate_keeper: m_gk.clone(),
            y_gate_keeper: y_gk.clone(),
            lefting: 3,
            acum: vec![]
        }
        .start()
        .send(SynonymRequest{ target: w.clone() }).await.unwrap();
    }

    println!("stopping system...");
    System::current().stop();
}