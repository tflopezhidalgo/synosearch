pub trait Parser {
    fn parse(&self, target: String) -> String;
}

/* TODO. refactor a módulos separados */
pub struct DummyProvider {
    pub url: String,
}

impl Parser for DummyProvider {
    fn parse(&self, _target: String) -> String {
        return "fake synonym".to_string();
    }
}
