fn parse_thesaurus(contents: String) -> Vec<String> {
    let vec_class = contents.split("e1ccqdb60\">").collect::<Vec<&str>>();
    let vec_ul = vec_class[1].split("</ul>").collect::<Vec<&str>>();
    let vec_il = vec_ul[0].split("<!-- -->").collect::<Vec<&str>>();
 
    let mut vec = Vec::new();
    for s in vec_il {
        if s.contains("eh475bn0\">") {
            let vec_data = s.split("eh475bn0\">").collect::<Vec<&str>>();
            let word = vec_data[1].replace("%20", " ").replace("&#x27;", "'");
            vec.push(word);
        }
    }
    return vec;
}

pub fn request_thesaurus(word: &str) -> Vec<String> {
    let url = format!("https://www.thesaurus.com/browse/{}", word);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    return parse_thesaurus(body);
} 
