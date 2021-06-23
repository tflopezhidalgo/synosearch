mod parsing;
use parsing::{
    //ThesaurusProvider, 
    //YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};

use actix::{Actor, Context, System};

struct MyActor{
    word: String,
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let p3 = &MarianWebsterProvider {url: "".to_string()};
        println!("{:?}",p3.parse(self.word.clone()));        
        System::current().stop(); // <- stop system
    }
}

fn main() {
    let system = System::new();
    let word = "car".to_string();
    let _addr = system.block_on(async { MyActor{word: word}.start() });

    //system.run(); 
}

/*
fn main() {

    let p1 = &ThesaurusProvider {url: "".to_string()};
    let p2 = &YourDictionaryProvider {url: "".to_string()};
    let p3 = &MarianWebsterProvider {url: "".to_string()};

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for p in providers {
        println!("{:?}", p.parse("car".to_string()));
    }
}
*/
