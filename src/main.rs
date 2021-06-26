mod parsing;

use std::sync::Arc;
use actix::prelude::*;
use actix::{Actor, Context, System, SyncContext};
use std::time::Instant;

use parsing::{
    ThesaurusProvider,
    MarianWebsterProvider,
    YourDictionaryProvider,
    Parser
};

static MIN_TIME_REQUESTS_SECS: u64 = 1;
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
        let syn = Arc::new(worker.parse(tmp));
        msg.source_addr.do_send(SVec{ msg: syn, web: Arc::new("Theaurus".to_string()) });
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
        let syn = Arc::new(worker.parse(tmp));
        msg.source_addr.do_send(SVec{ msg: syn, web: Arc::new("YourDictonary".to_string()) });
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
        let syn = Arc::new(worker.parse(tmp));
        msg.source_addr.do_send(SVec{ msg: syn, web: Arc::new("Marian".to_string()) });
    }
}

struct TWordGateKeeper { 
    worker: Arc<Addr<TheaurusWorker>>,
}

impl Actor for TWordGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for TWordGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);

        self.worker.do_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() });
    }
}

struct YourDictionaryGateKeeper {
    worker: Arc<Addr<YourDictionaryWorker>> 
}

impl Actor for YourDictionaryGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for YourDictionaryGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);

        self.worker.do_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() });
    }
}

struct MarianWebGateKeeper { 
    worker: Arc<Addr<MarianWebsterWorker>> 
}

impl Actor for MarianWebGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for MarianWebGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);

        self.worker.do_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() });
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
        self.t_gate_keeper.do_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg.clone() });
        self.y_gate_keeper.do_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg.clone() });
        self.m_gate_keeper.do_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg.clone() });
    }
}

impl Handler<SVec> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, msg: SVec, _ctx: &mut Context<Self>) -> Self::Result {
        self.lefting -= 1;
        self.acum.extend_from_slice(&msg.msg.clone());
        if self.lefting == 0 {
            println!("Palabra: {:?} tiene sinónimos:", self.target);
            println!("{:?}", self.acum.join(", "));
        }
    }
}


fn main() {
    let system = System::new();

    let mut words = vec!();

    let w1 = Arc::new("house".to_string());
    let w2 = Arc::new("car".to_string());
    let w3 = Arc::new("dog".to_string());

    words.push(w1.clone());
    words.push(w2.clone());
    words.push(w3.clone());
    
    system.block_on(async {
        let thesaurus_worker = Arc::new(SyncArbiter::start(5, || TheaurusWorker{}));
        let marian_worker = Arc::new(SyncArbiter::start(5, || MarianWebsterWorker{}));
        let your_dict_worker = Arc::new(SyncArbiter::start(5, || YourDictionaryWorker{}));

        let t_gk = Arc::new(TWordGateKeeper{worker: thesaurus_worker.clone()}.start());
        let m_gk = Arc::new(MarianWebGateKeeper{worker: marian_worker.clone()}.start());
        let y_gk = Arc::new(YourDictionaryGateKeeper{ worker: your_dict_worker.clone()}.start());

        for w in words {
            PerWordWorker { 
                target: w.clone(), 
                t_gate_keeper: t_gk.clone(),
                m_gate_keeper: m_gk.clone(),
                y_gate_keeper: y_gk.clone(),
                lefting: 3,
                acum: vec![]
            }
            .start()
            .do_send(Msg{ msg: w.clone() });
        }
    });

    system.run().unwrap();

    println!("stopping system...");
    System::current().stop();
}
