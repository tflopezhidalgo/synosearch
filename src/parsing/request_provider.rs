use std::fmt::Display;
use std::sync::Arc;

use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

use crate::Logger;

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
        let res = match Client::new()
            .get(self.url.clone())
            .header(USER_AGENT, APP_USER_AGENT)
            .send()
        {
            Ok(request) => request,
            Err(error) => panic!("Error request from {}: {:?}", self.url, error),
        };

        self.logger
            .info(format!("[{}] Requested for: {}", self, self.url));

        match res.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request from {}: {:?}", self.url, error),
        }
    }
}
