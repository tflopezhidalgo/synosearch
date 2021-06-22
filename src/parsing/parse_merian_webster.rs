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
            //println!("{}", _word);
            vec.push(word);
        }
    }
    return vec;
}

pub fn request_merian_webster(word: &str) -> Vec<String> {
    let url = format!("https://www.merriam-webster.com/thesaurus/{}", word);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    return parse_merian_webster(body);
}
