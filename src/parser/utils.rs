use crate::lexer::tokens::{Token, TokenKind};

use super::parser::Parser;

#[derive(Debug)]
pub struct TokenStream {
    input: Vec<Token>,
    position: usize,
}

impl TokenStream {
    /// Creates a new TokenStream from a generic collection.
    pub fn new(input: Vec<Token>) -> Self {
        TokenStream {
            input: input,  // Convert the String into a Vec<Token>
            position: 0,
        }
    }

    /// Peeks at the current element without consuming it.
    pub fn peek(&self) -> Option<&Token> {
        self.input.get(self.position)
    }

    /// Peeks at the next element without consuming.
    pub fn peek_next(&self) -> Option<&Token> {
        self.input.get(self.position + 1)
    }

    pub fn previous(&self) -> Option<Token> {
        if self.position >= 0 {
            let previous = self.input[self.position - 1].clone();
            Some(previous)
        } else {
            None
        }
    }

    /// Consumes the current element and moves the position forward.
    pub fn next(&mut self) -> Option<Token> {
        if self.position < self.input.len() {
            let current = self.input[self.position].clone();
            self.position += 1;
            Some(current)
        } else {
            None
        }
    }

    pub fn consume(&mut self, token_kind: TokenKind, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.unwrap().kind == token_kind {
            self.next();
            let token = self.previous();
            Ok(token.unwrap())
        } else {
            Err(msg.to_string())
        }
    }

    /// Checks if the stream has reached the end.
    pub fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

pub fn match_tokens(parser: &mut Parser, typs: &[TokenKind]) -> bool {
    for typ in typs {
        if match_token(parser, typ) {
            return true;
        }
    }
    false
}

pub fn match_token(parser: &mut Parser, typ: &TokenKind) -> bool {
    if parser.tokens.is_eof() {
        false
    } else {
        if parser.tokens
            .peek()
            .unwrap()
            .kind == *typ {
                parser.tokens.next();
                true
        } else {
            false
        }
    }
}