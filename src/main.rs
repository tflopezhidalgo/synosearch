//mod parsing; 
#[path = "parsing/parse_thesaurus.rs"] mod parse_thesaurus;
#[path = "parsing/parse_merian_webster.rs"] mod parse_merian_webster;
#[path = "parsing/parse_your_dictionary.rs"] mod parse_your_dictionary;

//use parsing::{DummyProvider, Parser};
/* cantidad máxima de pedidos a webs de forma concurrente */
//const MAX_REQ_CONCURRENCY: u32 = 1;

/* TODO tomar params desde línea de comandos */
/* TODO tiempo de espera entre dos requests consecutivas al mismo sitio */

fn main() {
    /*
    let temp_1 = DummyProvider{
        url: String::from("http://google.com")
    };
    let temp_2 = DummyProvider{
        url: String::from("http://yahoo.com")
    };

    let providers: Vec<&Parser> = std::vec![&temp_1, &temp_2];

    for p in providers {
        println!("{}", p.parse("car".to_string()));
    }
    */
    let word = "car";
    parse_thesaurus::request_thesaurus(word);
    parse_merian_webster::request_merian_webster(word);
    parse_your_dictionary::request_your_dictionary(word);
}
