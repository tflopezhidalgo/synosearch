use std::fs;

fn parse_thesaurus() {
    let contents = fs::read_to_string("scraper/thesaurus.html")
        .expect("Something went wrong reading the file");

    let vec_class = contents.split("<ul class=\"css-1xohnkh e1ccqdb60\">").collect::<Vec<&str>>();
    let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
    let vec_il = vec_ul[0].split("<!-- -->").collect::<Vec<&str>>();

    for s in vec_il {
        if s.contains("<li><a font-weight=\"inherit\" href=\"https://www.thesaurus.com/browse/") {
            let vec_data = s.split("<li><a font-weight=\"inherit\" href=\"https://www.thesaurus.com/browse/").collect::<Vec<&str>>();
            let vec_word = vec_data[1].split("\" data-linkid=\"nn1ov4\"").collect::<Vec<&str>>();
            println!("{}", vec_word[0].replace("%20", " "));
        }
    }   
}
