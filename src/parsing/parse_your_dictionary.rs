use reqwest::header::USER_AGENT;
const APP_USER_AGENT: &str = "curl/7.68.0";

fn parse_your_dictionary(contents: String) {
    let vec_class = contents.split("<div class=\"single-synonym-wrapper\" ").collect::<Vec<&str>>();
    let vec_ul = vec_class[1].split("</span></button></div></div></div> <!----></div></div> <!----></div></div>").collect::<Vec<&str>>();
    let vec_span = vec_ul[0].split("<!---->").collect::<Vec<&str>>(); 

    for s in vec_span {
        if s.contains("class=\"synonym-link\" data-v-b5c08d74>") {
            let split_word = s.split("class=\"synonym-link\" data-v-b5c08d74>").collect::<Vec<&str>>();
            let split_link = split_word[1].split("</").collect::<Vec<&str>>();
            println!("{}", split_link[0]);
        }
    }
}

pub fn request_your_dictionary(word: &str) {
    
    let client = reqwest::blocking::Client::new();

    let url = format!("https://thesaurus.yourdictionary.com/{}", word);
    let res = client.get(url)
        .header(USER_AGENT, APP_USER_AGENT)
        .send();

    let body = res.unwrap().text().unwrap();

    parse_your_dictionary(body);
}
