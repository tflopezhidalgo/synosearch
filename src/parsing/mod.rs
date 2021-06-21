pub trait Parser {
    fn parse(&self, target: String) -> String;
}

pub struct Dummy2 {
    pub url: String,
}

impl Parser for Dummy2 {
    fn parse(&self, target: String) -> String {
        return "dummy2".to_string();
    }
}

