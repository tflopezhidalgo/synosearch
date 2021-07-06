use std::sync::Arc;
use std::fmt::Display;

use reqwest::header::USER_AGENT;
use reqwest::blocking::Client;

use crate::Logger;

const MESSAGE_INIT: &str = "Get request from";
const MESSAGE_GET_CONTEXT: &str = "Get context request from";
const APP_USER_AGENT: &str = "curl/7.68.0";

pub struct RequestProvider {
    url: String,
    logger: Arc<Logger>,
}

impl Display for RequestProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestProvider")
    }
}

impl RequestProvider {
    pub fn new(url: String, logger: Arc<Logger>) -> Self {
        RequestProvider { url, logger }
    }

    pub fn make_request(&self) -> String {
        self.logger.info(format!("{} URL: {}", MESSAGE_INIT, self.url));

        let res = match Client::new().get(self.url.clone()).header(USER_AGENT, APP_USER_AGENT).send() {
            Ok(request) => request,
            Err(error) => panic!("Error request from {}: {:?}", self.url, error),
        };

        self.logger.info(format!("{} URL: {}", MESSAGE_GET_CONTEXT, self.url));

        match res.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request from {}: {:?}", self.url, error),
        }
    }
}
