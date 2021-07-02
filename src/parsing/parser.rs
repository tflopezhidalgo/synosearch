#[path = "../parsing/request_provider.rs"]
mod request_provider;
use request_provider::RequestProvider;

use std::sync::Arc;

use crate::Logger;


const APP_USER_AGENT: &str = "curl/7.68.0";
const MESSAGE_RETURN_SYNONIMOUS: &str = "Return synonimous from";

pub trait Parser {
    fn parse(&self, target: String) -> Vec<String>;
}

/* -- theaurus -- */

pub struct ThesaurusProvider {
    logger: Arc<Logger>,
}

impl ThesaurusProvider {
    pub fn new(logger: Arc<Logger>) -> Self {
        ThesaurusProvider { logger }
    }
}

const URL_THERASAURUS: &str = "https://www.thesaurus.com/browse/";

impl Parser for ThesaurusProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_THERASAURUS, target);
        let contents = RequestProvider::new(url.clone(), self.logger.clone()).
            make_request();

        let vec_class = contents.split("e1ccqdb60\">").collect::<Vec<&str>>();
        if vec_class.len() == 1 {
            return Vec::new();
        }
        let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
        let vec_il = vec_ul[0].split("<!-- -->").collect::<Vec<&str>>();

        let mut vec = Vec::new();
        for s in vec_il {
            if s.contains("eh475bn0\">") {
                let vec_data = s.split("eh475bn0\">").collect::<Vec<&str>>();
                let target = vec_data[1].replace("%20", " ").replace("&#x27;", "'");
                vec.push(target);
            }
        }
        self.logger.info(format!(
            "{} Therasaurus, WORD: {}",
            MESSAGE_RETURN_SYNONIMOUS, target
        ));
        return vec;
    }
}

/* -- yourdictonary -- */

pub struct YourDictionaryProvider {
    logger: Arc<Logger>,
}

impl YourDictionaryProvider {
    pub fn new(logger: Arc<Logger>) -> Self {
        YourDictionaryProvider { logger }
    }
}

const URL_YOURDICTIONARY: &str = "https://thesaurus.yourdictionary.com/";

impl Parser for YourDictionaryProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_YOURDICTIONARY, target);
        let contents = RequestProvider::new(url.clone(), self.logger.clone()).
            make_request_client(APP_USER_AGENT);
        
        let vec_class = contents
            .split("<div class=\"single-synonym-wrapper\" ")
            .collect::<Vec<&str>>();
        if vec_class.len() == 1 {
            return Vec::new();
        }
        let vec_ul = vec_class[1]
            .split("</span></button></div></div></div> <!----></div></div> <!----></div></div>")
            .collect::<Vec<&str>>();
        let vec_span = vec_ul[0].split("<!---->").collect::<Vec<&str>>();

        let mut vec = Vec::new();
        for s in vec_span {
            if s.contains("class=\"synonym-link\" data-v-b5c08d74>") {
                let split_word = s
                    .split("class=\"synonym-link\" data-v-b5c08d74>")
                    .collect::<Vec<&str>>();
                let split_link = split_word[1].split("</").collect::<Vec<&str>>();
                vec.push(split_link[0].to_string());
            }
        }
        self.logger.info(format!(
            "{} YourDictionary, WORD: {}",
            MESSAGE_RETURN_SYNONIMOUS, target
        ));
        return vec;
    }
}

/* -- marian webster -- */

pub struct MerriamWebsterProvider {
    logger: Arc<Logger>,
}

impl MerriamWebsterProvider {
    pub fn new(logger: Arc<Logger>) -> Self {
        MerriamWebsterProvider { logger }
    }
}

const URL_MERRIAM_WEBSTER: &str = "https://www.merriam-webster.com/thesaurus/";

impl Parser for MerriamWebsterProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_MERRIAM_WEBSTER, target);
        let contents = RequestProvider::new(url.clone(), self.logger.clone()).
            make_request();

        let vec_class = contents
            .split("<ul class=\"mw-list\">")
            .collect::<Vec<&str>>();
        if vec_class.len() == 1 {
            return Vec::new();
        }
        let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
        let vec_il = vec_ul[0].split("<li>").collect::<Vec<&str>>();

        let mut vec = Vec::new();
        for s in vec_il {
            let data = s.replace("\n", "");
            let vec_data = data.split("\">").collect::<Vec<&str>>();

            if vec_data[0].contains("<a class=\"\" href=\"/thesaurus/") {
                let word = vec_data[0]
                    .replace("<a class=\"\" href=\"/thesaurus/", "")
                    .replace(" ", "")
                    .replace("%20", " ");
                vec.push(word);
            }
        }
        self.logger.info(format!(
            "{} MarrianWebster, WORD: {}",
            MESSAGE_RETURN_SYNONIMOUS, target
        ));
        return vec;
    }
}
