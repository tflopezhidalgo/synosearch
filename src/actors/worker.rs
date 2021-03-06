use actix::prelude::Handler;
use actix::{Actor, SyncContext};

use std::sync::Arc;

use crate::main_actors::AvailableParsers;

use super::messages::{SynonymsResult, WorkerSynonymsRequest};

#[path = "../parsing/parser.rs"]
mod parser;

use parser::{MerriamWebsterProvider, Parser, ThesaurusProvider, YourDictionaryProvider};

/// Worker actor. Used in a pool of actors.
/// Responsible for request to the synonyms page and 
/// parse the content of result.
pub struct Worker;

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

/// Handles basic synonym request. It is, make the 
/// request to the page determined by the parser type and 
/// parsing the result content.
impl Handler<WorkerSynonymsRequest> for Worker {
    type Result = ();

    fn handle(
        &mut self,
        request: WorkerSynonymsRequest,
        _: &mut SyncContext<Self>,
    ) -> Self::Result {
        let parser: Option<Box<dyn Parser>> = match *request.parser {
            AvailableParsers::Thesaurus => {
                Some(Box::new(ThesaurusProvider::new(request.logger.clone())))
            }
            AvailableParsers::YourDictionary => Some(Box::new(YourDictionaryProvider::new(
                request.logger.clone(),
            ))),
            AvailableParsers::MerriamWebster => Some(Box::new(MerriamWebsterProvider::new(
                request.logger.clone(),
            ))),
        };

        request.logger.info(format!(
            "[{:?}] Worker making request for {:?}",
            *request.parser,
            (&request.target).to_string()
        ));

        let synonyms = Arc::new(parser.unwrap().parse((&request.target).to_string()));

        match request.response_addr.try_send(SynonymsResult { synonyms }) {
            Ok(_) => {}
            Err(_) => panic!("Error al enviar resultados de sinonimos"),
        }
    }
}
