use actix::prelude::*;

use std::sync::Arc;

use crate::logger::Logger;
use crate::main_actors::AvailableParsers;

#[derive(Message)]
#[rtype(result = "()")]
/// Message that increments the value of the counter actor.
pub struct Increment;

#[derive(Message)]
#[rtype(result = "()")]
/// Request for word synonyms.
pub struct SynonymRequest {
    pub target: Arc<String>,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Request for the gatekeeper.
pub struct GatekeeperRequest {
    pub target: Arc<String>,
    pub response_addr: Arc<Recipient<SynonymsResult>>,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Requests for the pool of workers.
pub struct WorkerSynonymsRequest {
    pub target: Arc<String>,
    pub response_addr: Arc<Recipient<SynonymsResult>>,
    pub parser: Arc<AvailableParsers>,
    pub logger: Arc<Logger>,
}

#[derive(Message)]
#[rtype(result = "()")]
/// Vec<String> representing synonyms for a word.
pub struct SynonymsResult {
    pub synonyms: Arc<Vec<String>>,
}
