use crate::{actors::*, logger::Logger};
use actix::prelude::*;
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SynonymRequest {
    pub target: Arc<String>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct GatekeeperRequest {
    pub target: Arc<String>,
    pub response_addr: Arc<Addr<PerWordWorker>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WorkerSynonymsRequest {
    pub target: Arc<String>,
    pub response_addr: Arc<Addr<PerWordWorker>>,
    pub parser_key: String,
    pub logger: Arc<Logger>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SynonymsResult {
    pub synonyms: Arc<Vec<String>>,
}
