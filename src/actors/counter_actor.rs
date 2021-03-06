use std::fmt::Display;
use std::sync::Arc;

use actix::prelude::*;
use actix::{Actor, Context};

use crate::logger::Logger;

use super::messages::Increment;

/// Actor responsible for carry the count of current
/// finished words. When the count (all the words) has been reached
/// this actor will stop the entire system.
pub struct CounterActor {
    /// Limit to reach the count.
    pub limit: u32,

    /// Actual count of processed words.
    pub count: u32,

    /// Reference to the global logger
    pub logger: Arc<Logger>,
}

impl Actor for CounterActor {
    type Context = Context<Self>;
}

impl Display for CounterActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CounterActor")
    }
}

/// Handler for the Increment message. This message is received
/// when one of the PerWordWorker finishes the processing of its word.
/// When we reach the limit stop the entire system.
impl Handler<Increment> for CounterActor {
    type Result = ();

    fn handle(&mut self, _: Increment, _: &mut Context<Self>) -> Self::Result {
        self.logger
            .info(format!("[{}] Received Increment message", self));
        self.count += 1;
        if self.count == self.limit {
            self.logger
                .info(format!("[{}] Stopping current system", self));
            System::current().stop();
        }
    }
}
