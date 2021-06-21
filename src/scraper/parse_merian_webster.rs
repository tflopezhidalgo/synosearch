use std::fs;

fn parse_merian_webster() {
    let contents = fs::read_to_string("scraper/merriam_webster.html")
        .expect("Something went wrong reading the file");

    let vec_class = contents.split("<ul class=\"mw-list\">").collect::<Vec<&str>>();
    let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
    let vec_il = vec_ul[0].split("<li>").collect::<Vec<&str>>();

    for s in vec_il {
        let word = s.replace("\n", "");
        let vec_word = word.split("\">").collect::<Vec<&str>>();

        if vec_word[0].contains("<a class=\"\" href=\"/thesaurus/") {
            let _word = vec_word[0].replace("<a class=\"\" href=\"/thesaurus/", "")
                .replace(" ", "").replace("%20", " ");
            
            println!("{}", _word);
        }
    }}

