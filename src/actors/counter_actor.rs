#[path = "../utils/counter.rs"]
mod counter;

#[path = "../parsing/parser.rs"]
mod parser;

use actix::prelude::*;
use actix::{Actor, Context};

use super::messages::Increment;

/// Actor responsible for carry the count of current
/// finished words. When the count (all the words) has been reached
/// this actor will stop the entire system.
pub struct CounterActor {
    /// Limit to reach the count.
    pub limit: u32,

    /// Actual count of processed words.
    pub count: u32,
}

impl Actor for CounterActor {
    type Context = Context<Self>;
}

impl Handler<Increment> for CounterActor {
    type Result = ();

    fn handle(&mut self, _: Increment, _: &mut Context<Self>) -> Self::Result {
        self.count += 1;
        if self.count == self.limit {
            System::current().stop();
        }
    }
}
