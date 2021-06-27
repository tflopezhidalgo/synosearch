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

/* Message */
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

/* Actors Workers */

struct TheaurusWorker;
struct YourDictionaryWorker;
struct MarianWebsterWorker;


impl Actor for TheaurusWorker {
    type Context = SyncContext<Self>;
}

impl Actor for YourDictionaryWorker {
    type Context = SyncContext<Self>;
}

impl Actor for MarianWebsterWorker {
    type Context = SyncContext<Self>;
}

impl Handler<AddrMsg> for TheaurusWorker {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */
        let worker = &ThesaurusProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        let syn = Arc::new(worker.parse(tmp));
        msg.source_addr.do_send(SVec{ msg: syn, web: Arc::new("Theaurus".to_string()) });
    }
}

impl Handler<AddrMsg> for YourDictionaryWorker {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */
        let worker = &YourDictionaryProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        let syn = Arc::new(worker.parse(tmp));
        msg.source_addr.do_send(SVec{ msg: syn, web: Arc::new("YourDictonary".to_string()) });
    }
}

impl Handler<AddrMsg> for MarianWebsterWorker {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */
        let worker = &MarianWebsterProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        let syn = Arc::new(worker.parse(tmp));
        msg.source_addr.do_send(SVec{ msg: syn, web: Arc::new("Marian".to_string()) });
    }
}


struct WordGateKeeper {
    //TODO. polymorphic vector
    t_worker: Arc<Addr<TheaurusWorker>>, 
    y_worker: Arc<Addr<YourDictionaryWorker>>, 
    m_worker: Arc<Addr<MarianWebsterWorker>>, 
}
// workers: Vec<Box<Arc<Addr<ParserWorkers>>>>

impl Actor for WordGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for WordGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);

        // for w in workers {}
        self.t_worker.do_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() });
        self.y_worker.do_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() });
        self.m_worker.do_send(AddrMsg_{ source_addr: source_addr.clone(), msg: msg.msg.clone() });
    }
}

struct PerWordWorker {
    gate_keeper: Arc<Addr<WordGateKeeper>>,
}

impl Actor for PerWordWorker {
    type Context = Context<Self>;
}

impl Handler<Msg> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Asking synonym for {:?}", msg.msg);
        self.gate_keeper.do_send(AddrMsg{ source_addr: _ctx.address(), msg: msg.msg })
    }
}

impl Handler<SVec> for PerWordWorker {
    type Result = ();

    fn handle(&mut self, msg: SVec, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{:p} recibio {:?}", self, msg.msg.len());
    }
}