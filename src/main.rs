mod parsing;

use std::sync::Arc;
use actix::prelude::*;
use actix::{Actor, Context, System, SyncContext};
use std::time::Instant;
use std::thread::{self, sleep};

use parsing::{
    ThesaurusProvider,
    MarianWebsterProvider,
    YourDictionaryProvider,
    Parser
};

static MIN_TIME_REQUESTS_SECS: u64 = 10;
static MAX_CONCURRENCY: isize = 5;

/* Messages */
#[derive(Message)]
#[rtype(result = "()")]
struct AddrMsg {
    msg: Arc<String>,
    source_addr: Addr<PerWordWorker>
}

#[derive(Message)]
#[rtype(result = "()")]
struct Msg {
    msg: Arc<String>,
}


#[derive(Message)]
#[rtype(result = "()")]
struct AddrMsg_ {
    msg: Arc<String>,
    source_addr: Arc<Addr<PerWordWorker>>
}

#[derive(Message)]
#[rtype(result = "()")]
struct SVec {
    web: Arc<String>,
    msg: Arc<Vec<String>>,
}

/* Actors */

struct TheaurusWorker;

impl Actor for TheaurusWorker {
    type Context = SyncContext<Self>;
}

impl Handler<AddrMsg_> for TheaurusWorker {

    type Result = ();

    fn handle(&mut self, msg: AddrMsg_, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */
        let worker = &ThesaurusProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        println!("*** Worker [T] started parsing {:?}", tmp.clone());
        let syn = Arc::new(worker.parse(tmp.clone()));
        println!("Worker [T] sending to origin synonyms for word {:?}", tmp.clone());
        let result = msg.source_addr.try_send(SVec{ msg: syn, web: Arc::new("Theaurus".to_string()) });
        println!("{:?}", result);
    }
}

struct YourDictionaryWorker;

impl Actor for YourDictionaryWorker {
    type Context = SyncContext<Self>;
}

impl Handler<AddrMsg_> for YourDictionaryWorker {

    type Result = ();

    fn handle(&mut self, msg: AddrMsg_, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */
        let worker = &YourDictionaryProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        println!("*** Worker [W] started parsing {:?}", tmp.clone());
        let syn = Arc::new(worker.parse(tmp.clone()));
        println!("Worker [W] sending to origin synonyms for word {:?}", tmp.clone());
        msg.source_addr.try_send(SVec{ msg: syn, web: Arc::new("YourDictonary".to_string()) });
    }
}

struct MarianWebsterWorker;

impl Actor for MarianWebsterWorker {
    type Context = SyncContext<Self>;
}

impl Handler<AddrMsg_> for MarianWebsterWorker {

    type Result = ();

    fn handle(&mut self, msg: AddrMsg_, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */

        let worker = &MarianWebsterProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        println!("*** Worker [M] started parsing {:?}", tmp.clone());
        let syn = Arc::new(worker.parse(tmp.clone()));
        println!("Worker [M] sending to origin synonyms for word {:?}", tmp.clone());
        msg.source_addr.try_send(SVec{ msg: syn, web: Arc::new("Marian".to_string()) }).unwrap();
    }
}

struct TWordGateKeeper { 
    worker: Arc<Addr<TheaurusWorker>>,
    last: std::time::Instant,
}

impl Actor for TWordGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for TWordGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);
        println!("[T] handling {:?}", msg.msg.clone());

        let elapsed = std::time::Instant::now().duration_since(self.last).as_secs();
        if  elapsed < MIN_TIME_REQUESTS_SECS {
            println!("[T] Sleeping by {:?} secs.", (MIN_TIME_REQUESTS_SECS - elapsed));
            std::thread::sleep(std::time::Duration::from_secs(MIN_TIME_REQUESTS_SECS - elapsed));
            println!("[T] Awaking");
        }

        println!("[T] Making request for {:?}", msg.msg.clone());
        self.worker.try_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() }).unwrap();
        self.last = std::time::Instant::now();
    }
}

struct YourDictionaryGateKeeper {
    worker: Arc<Addr<YourDictionaryWorker>>,
    last: std::time::Instant,
}

impl Actor for YourDictionaryGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for YourDictionaryGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);
        println!("[Y] handling {:?}", msg.msg.clone());

        let elapsed = std::time::Instant::now().duration_since(self.last).as_secs();
        if  elapsed < MIN_TIME_REQUESTS_SECS {
            println!("[Y] Sleeping by {:?} secs.", (MIN_TIME_REQUESTS_SECS - elapsed));
            std::thread::sleep(std::time::Duration::from_secs(MIN_TIME_REQUESTS_SECS - elapsed));
            println!("[Y] Awaking");
        }

        println!("[Y] Making request for {:?}", msg.msg.clone());
        self.worker.try_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() }).unwrap();
        self.last = std::time::Instant::now();
    }
}

struct MarianWebGateKeeper { 
    worker: Arc<Addr<MarianWebsterWorker>>,
    last: std::time::Instant,
}

impl Actor for MarianWebGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for MarianWebGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);
        println!("[M] handling {:?}", msg.msg.clone());

        let elapsed = std::time::Instant::now().duration_since(self.last).as_secs();
        if  elapsed < MIN_TIME_REQUESTS_SECS {
            println!("[M] Sleeping by {:?} secs.", (MIN_TIME_REQUESTS_SECS - elapsed));
            std::thread::sleep(std::time::Duration::from_secs(MIN_TIME_REQUESTS_SECS - elapsed));
            println!("[M] Awaking");
        }

        println!("[M] Making request for {:?}", msg.msg.clone());
        self.worker.try_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() }).unwrap();
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

impl Handler<Msg> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Asking synonym for {:?}", msg.msg);
        self.target = msg.msg.clone();
        self.t_gate_keeper.try_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg.clone() }).unwrap();
        println!("Sended to [T]");
        self.y_gate_keeper.try_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg.clone() }).unwrap();
        println!("Sended to [Y]");
        self.m_gate_keeper.try_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg.clone() }).unwrap();
        println!("Sended to [M]");
    }
}

impl Handler<SVec> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, msg: SVec, _ctx: &mut Context<Self>) -> Self::Result {
        println!("*** sinonimos para {:?} recibidos", self.target);
        let mut tmp = self.lefting;
        tmp -= 1;
        self.acum.extend_from_slice(&msg.msg.clone());
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
    //let w2 = Arc::new("cat".to_string());
    //let w3 = Arc::new("car".to_string());

    words.push(w1.clone());
    //words.push(w2.clone());
    //words.push(w3.clone());
    
    let thesaurus_worker = Arc::new(SyncArbiter::start(5, || TheaurusWorker{}));
    let marian_worker = Arc::new(SyncArbiter::start(5, || MarianWebsterWorker{}));
    let your_dict_worker = Arc::new(SyncArbiter::start(5, || YourDictionaryWorker{}));

    let m_gk = Arc::new(
        MarianWebGateKeeper{
            worker: marian_worker.clone(),
            last: std::time::Instant::now() - std::time::Duration::from_secs(10000)
        }.start());

    let t_gk = Arc::new(
        TWordGateKeeper{
            worker: thesaurus_worker.clone(),
            last: std::time::Instant::now() - std::time::Duration::from_secs(10000)
        }.start());


    let y_gk = Arc::new(
        YourDictionaryGateKeeper{ 
            worker: your_dict_worker.clone(),
            last: std::time::Instant::now() - std::time::Duration::from_secs(10000)
        }.start());

//    let mut w_workers: Vec<_> = vec!();

    let a = PerWordWorker { 
        target: Arc::new("".to_string()).clone(), 
        t_gate_keeper: t_gk.clone(),
        m_gate_keeper: m_gk.clone(),
        y_gate_keeper: y_gk.clone(),
        lefting: 3,
        acum: vec![]
    }
    .start();

    for w in words {
        println!("{:?} ***************** ", w);
        a.send(Msg{ msg: w.clone() }).await.unwrap();
    }

    //w_workers.iter().map(|x| &x.await).collect::<Vec<_>>();

    sleep(std::time::Duration::from_secs(100));

    println!("stopping system...");
    System::current().stop();
}
