use crate::lexer::utils::read_file;

pub struct Lexer {
    tokens: Vec<String>
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            tokens: vec![],
        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<String> {

        let contents = read_file(input);

        vec![]
    }
}