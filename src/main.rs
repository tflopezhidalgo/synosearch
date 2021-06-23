use actix::prelude::*;

fn parse_merian_webster(contents: String) -> Vec<String> {
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
            println!("{}", word);
            vec.push(word);
        }
    }
    return vec;
}

async fn request_merian_webster(word: String) -> Result<Vec<String>, reqwest::Error> {
    let url = format!("https://www.merriam-webster.com/thesaurus/{}", word);
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
        //.unwrap().text().unwrap();

    let response =  parse_merian_webster(body);
    return Ok(response);
} 

// this is our Message
// we have to define the response type (rtype)
#[derive(Message)]
#[rtype(result = "Vec<String>")]
struct Parse(String);

// Actor definition
struct Scraper;

impl Actor for Scraper {
    type Context = Context<Self>;
}

impl Handler<Parse> for Scraper {
    type Result = Vec<String>; // <- Message response type

    fn handle(&mut self, msg: Parse, _ctx: &mut Context<Self>) -> Self::Result {
        return request_merian_webster(msg.0);
    }
}

#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    let addr = Scraper.start();
    let res = addr.send(Parse("car".to_string())).await; // <- send message and get future for result

    match res {
        Ok(result) => println!("WORD: {:?}", result),
        _ => println!("Communication to the actor has failed"),
    }
}
