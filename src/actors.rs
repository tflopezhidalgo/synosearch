mod parsing;
use parsing::{
    MarianWebsterProvider, 
    Parser
};

use actix::{Actor, Context, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let p3 = &MarianWebsterProvider {url: "".to_string()};
        println!("{:?}",p3.parse("car".to_string()));        
        System::current().stop(); // <- stop system
    }
}

fn main() {
    let system = System::new();

    let _addr = system.block_on(async { MyActor.start() });

    //system.run(); 
}

