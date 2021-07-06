use std::sync::Arc;

use reqwest::header::USER_AGENT;

use crate::Logger;

const MESSAGE_INIT: &str = "Get request from";
const MESSAGE_GET_CONTEXT: &str = "Get context request from";

pub struct RequestProvider {
    url: String,
    logger: Arc<Logger>,
}

impl RequestProvider {
    pub fn new(url: String, logger: Arc<Logger>) -> Self {
        RequestProvider { url, logger }
    }

    pub fn make_request(&self) -> String {
        self.logger
            .info(format!("{} URL: {}", MESSAGE_INIT, self.url));
        let request = match reqwest::blocking::get(self.url.clone()) {
            Ok(request) => request,
            Err(error) => panic!("Error request from {}: {:?}", self.url, error),
        };

        self.logger
            .info(format!("{} URL: {}", MESSAGE_GET_CONTEXT, self.url));

        let contents = match request.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request from {:?}: {:?}", self.url, error),
        };
        return contents;
    }

    pub fn make_request_client(&self, user_agent: &str) -> String {
        self.logger
            .info(format!("{} URL: {}", MESSAGE_INIT, self.url));
        let client = reqwest::blocking::Client::new();
        let res = match client
            .get(self.url.clone())
            .header(USER_AGENT, user_agent)
            .send()
        {
            Ok(request) => request,
            Err(error) => panic!("Error request from {}: {:?}", self.url, error),
        };

        self.logger
            .info(format!("{} URL: {}", MESSAGE_GET_CONTEXT, self.url));
        let contents = match res.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request from {}: {:?}", self.url, error),
        };

        return contents;
    }
}
