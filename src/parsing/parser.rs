#[path = "../parsing/request_provider.rs"]
mod request_provider;
use request_provider::RequestProvider;

use std::fmt::Display;
use std::sync::Arc;

use crate::Logger;

const URL_THERASAURUS: &str = "https://www.thesaurus.com/browse/";
const URL_MERRIAM_WEBSTER: &str = "https://www.merriam-webster.com/thesaurus/";
const URL_YOURDICTIONARY: &str = "https://thesaurus.yourdictionary.com/";

/// Basic Parser trait. Who implements this trait should
/// provide a definition for the parse() function.
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

impl Display for ThesaurusProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThesaurusProvider")
    }
}

impl Parser for ThesaurusProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_THERASAURUS, target);
        let contents = RequestProvider::new(url.clone(), self.logger.clone()).make_request();

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
        self.logger
            .info(format!("[{}] Parsed content for: {}", self, target));
        vec
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

impl Display for YourDictionaryProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "YourDictionaryProvider")
    }
}

impl Parser for YourDictionaryProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_YOURDICTIONARY, target);
        let contents = RequestProvider::new(url.clone(), self.logger.clone()).make_request();

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
        self.logger
            .info(format!("[{}] Parsed content for: {}", self, target));
        vec
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

impl Display for MerriamWebsterProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MerriamWebsterProvider")
    }
}

impl Parser for MerriamWebsterProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_MERRIAM_WEBSTER, target);
        let contents = RequestProvider::new(url.clone(), self.logger.clone()).make_request();

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
        self.logger
            .info(format!("[{}] Parsed content for: {}", self, target));
        vec
    }
}
