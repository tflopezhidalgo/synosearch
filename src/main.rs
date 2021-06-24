mod parsing;
use core::time;
use std::thread::sleep;
use std::sync::Arc;

use parsing::{
    MarianWebsterProvider, 
    ThesaurusProvider,
    Parser
};

use actix::{Actor, Context, System, SyncContext};
use actix::prelude::*;

/* - Actor leyendo las palabras del archivo.
 *  - Actor que recibe una palabra a buscar.
 *  - Tres instancias únicas de sitios (gatekeeper)
 *  - Pool de workers.
 */

/* Messages */

#[derive(Message)]
#[rtype(result = "()")]
struct Msg {
    msg: Arc<String>,
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
struct ParsingMessage {
    msg: Arc<String>,
}

/* Actors */

struct TheaurusWorker { }

// TODO. sacarlo
impl Actor for TheaurusWorker {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        return
    }
}

impl Handler<ParsingMessage> for TheaurusWorker {

    type Result = Vec<String>;

    fn handle(&mut self, msg: ParsingMessage, _ctx: &mut SyncContext<Self>) -> Self::Result {
        /* Busca sinónimo para una palabra en un determinado sitio */
        let worker = &ThesaurusProvider { url: "".to_string() };

        let tmp = (*msg.msg).clone();
        return worker.parse(tmp);
    }
}

struct WordGateKeeper {
    //FIXME
    worker: Arc<Addr<TheaurusWorker>>
}

// TODO. sacarlo
impl Actor for WordGateKeeper {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<Msg> for WordGateKeeper {

    type Result = ();

    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Self>) -> Self::Result {
        self.worker.do_send(ParsingMessage{ msg: msg.msg });
    }
}


fn main() {
    let system = System::new();

    let mut words = vec!();

    let w1 = Arc::new("casa".to_string());
    let w2 = Arc::new("auto".to_string());
    let w3 = Arc::new("perro".to_string());

    words.push(w1.clone());
    words.push(w2.clone());
    words.push(w3.clone());

    let thesaurus_worker = Arc::new(SyncArbiter::start(5, || TheaurusWorker{}));

    let t_worker = thesaurus_worker.clone();

    let gatekeeper = WordGateKeeper { worker: t_worker }.start();

    system.block_on(async {
        for w in words {
            gatekeeper.do_send(Msg{ msg: w.clone() });
        }
    });

    system.run().unwrap(); 
}
