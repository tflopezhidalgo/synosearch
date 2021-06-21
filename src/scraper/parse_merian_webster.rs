use std::fs;

fn parse_merian_webster() {
    let contents = fs::read_to_string("scraper/merriam_webster.html")
        .expect("Something went wrong reading the file");

    let split_class = contents.split("<ul class=\"mw-list\">");
    let vec_class = split_class.collect::<Vec<&str>>();
    
    let split_ul = vec_class[1].split("</ul>");
    let vec_ul = split_ul.collect::<Vec<&str>>();

    let split_il = vec_ul[0].split("<li>");
    let vec_il = split_il.collect::<Vec<&str>>();

    for s in vec_il {
        let word = s.replace("\n", "");
        let split_word = word.split("\">");
        let vec_word = split_word.collect::<Vec<&str>>();

        if vec_word[0].contains("<a class=\"\" href=\"/thesaurus/") {
            let _word = vec_word[0].replace("<a class=\"\" href=\"/thesaurus/", "")
                .replace(" ", "")
                .replace("%20", " ");
            
            println!("{}", _word);
        }
    }}

