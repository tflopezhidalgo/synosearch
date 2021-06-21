use std::fs;

fn parse_thesaurus() {
    let contents = fs::read_to_string("scraper/thesaurus.html")
        .expect("Something went wrong reading the file");

    let split_class = contents.split("<ul class=\"css-1xohnkh e1ccqdb60\">");
    let vec_class = split_class.collect::<Vec<&str>>();

    let split_ul = vec_class[1].split("</ul>");
    let vec_ul = split_ul.collect::<Vec<&str>>();

    let split_il = vec_ul[0].split("<!-- -->");
    let vec_il = split_il.collect::<Vec<&str>>();


    for s in vec_il {
        let word = s.replace("<li><a font-weight=\"inherit\" href=\"https://www.thesaurus.com/browse/", "")
                    .replace("\" data-linkid=\"nn1ov4\" class=\"css-1kg1yv8 eh475bn0\"", "")
                    .replace("\" data-linkid=\"nn1ov4\" class=\"css-1gyuw4i eh475bn0\"", "")
                    .replace("\" data-linkid=\"nn1ov4\" class=\"css-1n6g4vv eh475bn0\"", "")
                    .replace(" </a></li>", "")
                    .replace("%20", " ");

        let split_word = word.split(">");
        let vec_word = split_word.collect::<Vec<&str>>();
        println!("{}", vec_word[0])
    }
}
