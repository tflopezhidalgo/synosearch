pub struct DummyProvider {
    pub url: String,
}

impl Parser for DummyProvider {
    fn parse(&self, target: String) -> String {
        return "dummy synonym".to_string();
    }
}
