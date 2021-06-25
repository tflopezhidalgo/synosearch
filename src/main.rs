mod parsing;

use std::process;
use std::env;
use std::fs;

use parsing::{
    ThesaurusProvider, 
    YourDictionaryProvider, 
    MarianWebsterProvider, 
    Parser
};


fn try_mode() {
    let p1 = &ThesaurusProvider {url: "".to_string()};
    let p2 = &YourDictionaryProvider {url: "".to_string()};
    let p3 = &MarianWebsterProvider {url: "".to_string()};

    let providers: Vec<& dyn Parser> = vec![p1, p2, p3];

    for p in providers {
        println!("{:?}", p.parse("car".to_string()));
    }

}

fn read_file_into_vector(filename: String) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file"); 

    let words = contents.split("\n").collect::<Vec<&str>>();
    let mut vec = Vec::new();
    for w in words.into_iter() {
        vec.push(w.to_string());
    }
    return vec;
}

fn choose_mode(mode:String, filename: String) {
    let _words = read_file_into_vector(filename);
    if mode.eq("actors") {
        println!("actors");
    } else if mode.eq("threads") {
        println!("threads");
    } else {
        try_mode();
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        process::exit(-1);
    }
    choose_mode(args[1].clone(), args[2].clone());
}
