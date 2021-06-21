use std::fs;

fn parse_your_dictionary() {
    let contents = fs::read_to_string("scraper/your_dictionary.html")
        .expect("Something went wrong reading the file");

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

