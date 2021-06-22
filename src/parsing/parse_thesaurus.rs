fn parse_thesaurus(contents: String) {
    let vec_class = contents.split("<ul class=\"css-1xohnkh e1ccqdb60\">").collect::<Vec<&str>>();
    let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
    let vec_il = vec_ul[0].split("<!-- -->").collect::<Vec<&str>>();
    
    for s in vec_il {
        if s.contains("eh475bn0\">") {
            let vec_data = s.split("eh475bn0\">").collect::<Vec<&str>>();
            println!("{}", vec_data[1].replace("%20", " "));
        }
    } 
}

pub fn request_thesaurus(word: &str) {
    let url = format!("https://www.thesaurus.com/browse/{}", word);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    parse_thesaurus(body);
} 
