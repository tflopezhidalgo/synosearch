
use actix::prelude::*;

mod parsing;
use parsing::{
    MarianWebsterProvider, 
    Parser
};

// this is our Message
// we have to define the response type (rtype)
#[derive(Message)]
#[rtype(result = "Vec<String>")]
struct Word(String);

// Actor definition
struct Scraper;

impl Actor for Scraper {
    type Context = Context<Self>;
}

impl Handler<Word> for Scraper {
    type Result = Vec<String>; // <- Message response type

    fn handle(&mut self, word: Word, _ctx: &mut Context<Self>) -> Self::Result {
        let p3 = &MarianWebsterProvider {url: "".to_string()};
        return p3.parse(word.0);
    }
}

#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    let addr = Scraper.start();
    let res = addr.send(Word("car".to_string())).await; // <- send message and get future for result

    match res {
        Ok(result) => println!("WORD: {:?}", result),
        _ => println!("Communication to the actor has failed"),
    }
}
