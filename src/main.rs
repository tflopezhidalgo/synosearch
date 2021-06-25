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

struct WordGateKeeper {
    //TODO. polymorphic vector
    t_worker: Arc<Addr<TheaurusWorker>>, 
    y_worker: Arc<Addr<YourDictionaryWorker>>, 
    m_worker: Arc<Addr<MarianWebsterWorker>>, 
}

impl Actor for WordGateKeeper {
    type Context = Context<Self>;
}

impl Handler<AddrMsg> for WordGateKeeper {
    type Result = ();

    fn handle(&mut self, msg: AddrMsg, _ctx: &mut Context<Self>) -> Self::Result {
        let source_addr = Arc::new(msg.source_addr);

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

        let gatekeeper = Arc::new(
            WordGateKeeper {
                t_worker: thesaurus_worker.clone(),
                y_worker: your_dict_worker.clone(),
                m_worker: marian_worker.clone()
            }
            .start());

        let per_word_worker = PerWordWorker { gate_keeper: gatekeeper.clone() }.start();

        for w in words {
            per_word_worker.do_send(Msg{ msg: w.clone() });
        }
    });

    system.run().unwrap();

    System::current().stop();
}
