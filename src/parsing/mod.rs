use reqwest::header::USER_AGENT;
const APP_USER_AGENT: &str = "curl/7.68.0";

pub trait Parser {
    fn parse(&self, target: String) -> Vec<String>;
}

/* -- theaurus -- */

pub struct ThesaurusProvider;

const URL_THERASAURUS: &str = "https://www.thesaurus.com/browse/";

impl Parser for ThesaurusProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_THERASAURUS, target);
        let request = match reqwest::blocking::get(url) {
            Ok(request) => request,
            Err(error) => panic!("Error request from Therasaurus: {:?}", error)
        };

        let contents = match request.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request: {:?}", error)
        };

        let vec_class = contents.split("e1ccqdb60\">").collect::<Vec<&str>>();
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
        return vec;
    }
}


/* -- yourdictonary -- */

pub struct YourDictionaryProvider;

const URL_YOURDICTIONARY: &str = "https://thesaurus.yourdictionary.com/";

impl Parser for YourDictionaryProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}{}", URL_YOURDICTIONARY, target);
        let res = match client.get(url)
            .header(USER_AGENT, APP_USER_AGENT)
            .send() {
                Ok(request) => request,
                Err(error) => panic!("Error request from Therasaurus: {:?}", error)
            };

        let contents = match res.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request: {:?}", error)
        };

        let vec_class = contents.split("<div class=\"single-synonym-wrapper\" ").collect::<Vec<&str>>();
        let vec_ul = vec_class[1].split("</span></button></div></div></div> <!----></div></div> <!----></div></div>").collect::<Vec<&str>>();
        let vec_span = vec_ul[0].split("<!---->").collect::<Vec<&str>>(); 

        let mut vec = Vec::new();
        for s in vec_span {
            if s.contains("class=\"synonym-link\" data-v-b5c08d74>") {
                let split_word = s.split("class=\"synonym-link\" data-v-b5c08d74>").collect::<Vec<&str>>();
                let split_link = split_word[1].split("</").collect::<Vec<&str>>();
                vec.push(split_link[0].to_string());
            }
        }
        return vec;
    }
}


/* -- marian webster -- */

pub struct MarrianWebsterProvider;

const URL_MARRIAM_WEBSTER: &str = "https://www.merriam-webster.com/thesaurus/";

impl Parser for MarrianWebsterProvider {
    fn parse(&self, target: String) -> Vec<String> {
        let url = format!("{}{}", URL_MARRIAM_WEBSTER, target);
        let request = match reqwest::blocking::get(url) {
            Ok(request) => request,
            Err(error) => panic!("Error request from Therasaurus: {:?}", error)
        };

        let contents = match request.text() {
            Ok(contents) => contents,
            Err(error) => panic!("Error reading request: {:?}", error)
        };

        let vec_class = contents.split("<ul class=\"mw-list\">").collect::<Vec<&str>>();
        let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
        let vec_il = vec_ul[0].split("<li>").collect::<Vec<&str>>();

        let mut vec = Vec::new();
        for s in vec_il {
            let data = s.replace("\n", "");
            let vec_data = data.split("\">").collect::<Vec<&str>>();

            if vec_data[0].contains("<a class=\"\" href=\"/thesaurus/") {
                let word = vec_data[0].replace("<a class=\"\" href=\"/thesaurus/", "")
                    .replace(" ", "").replace("%20", " ");
                vec.push(word);
            }
        }
        return vec;
    }
}
