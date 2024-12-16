use std::{array::IntoIter, borrow::Borrow, collections::HashMap, iter::Peekable};

use crate::lexer::utils::read_file;

use super::tokens::{Token, TokenKind};

fn get_keywords_hashmap() -> HashMap<&'static str, TokenKind> {
    HashMap::from([
        ("and", TokenKind::And),
        ("class", TokenKind::Class),
        ("else", TokenKind::Else),
        ("false", TokenKind::False),
        ("true", TokenKind::True),
        ("fun", TokenKind::Fun),
        ("for", TokenKind::For),
        ("if", TokenKind::If),
        ("null", TokenKind::Null),
        ("or", TokenKind::Or),
        ("print", TokenKind::Print),
        ("return", TokenKind::Return),
        ("super", TokenKind::Super),
        ("this", TokenKind::This),
        ("let", TokenKind::Let),
        ("while", TokenKind::While),
        ("break", TokenKind::Break),
        ("continue", TokenKind::Continue)
    ])
}

pub struct Lexer {
    source_file: &'static str,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenKind>,
    position: (usize, usize),     // line, column
    current_position: (usize, usize)
}

impl Lexer {
    pub fn new(source: &'static str) -> Self {
        Lexer {
            source_file: source,
            tokens: vec![],
            keywords: get_keywords_hashmap(),
            position: (1, 1),
            current_position: (1, 1)
        }
    }

    pub fn tokenize(&mut self, source: &'static str) -> Result<Vec<Token>, String> {

        let mut contents = read_file(&source)
            .into_iter()
            .peekable();

        let mut tokens: Vec<Token> = vec![];
        let mut errors = vec![];

        while let Some(_) = contents.peek() {
            let token_result: Result<Option<Token>, String> = self.scan_token(&mut contents);

            match token_result {
                Ok(Some(token)) => {
                    tokens.push(token);
                }
                Ok(None) => {
                    continue // this is good behaviour I guess
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }

        tokens.push(Token::eof(
            String::from(self.source_file),
            self.position.0,
            self.position.1
        ));

        // this could be more rusty
        if errors.len() > 0 {
            let mut joined = String::new();
            for msg in &errors {
                joined.push_str(msg);
                joined.push_str("\n");
            }
            return Err(joined)
        }

        Ok(vec![])
    }
    
    fn scan_token<I>(&mut self, contents: &mut Peekable<I>) -> Result<Option<Token>, String>
    where
        I: Iterator<Item = char>
    {
        
        if let Some(char) = contents.next() {
            match char {
                // Handle single-character static_tokenkinds
                '(' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::LeftParen,
                    self.position.0,
                    self.position.1,
                ))),
                ')' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::RightParen,
                    self.position.0,
                    self.position.1,
                ))),
                '{' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::LeftBrace,
                    self.position.0,
                    self.position.1,
                ))),
                '}' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::RightBrace,
                    self.position.0,
                    self.position.1,
                ))),
                ',' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::Comma,
                    self.position.0,
                    self.position.1,
                ))),
                '.' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::Dot,
                    self.position.0,
                    self.position.1,
                ))),
                ';' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::Semicolon,
                    self.position.0,
                    self.position.1,
                ))),
                '+' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::Plus,
                    self.position.0,
                    self.position.1,
                ))),
                '-' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::Minus,
                    self.position.0,
                    self.position.1,
                ))),
                '*' => Ok(Some(Token::static_tokenkind(
                    self.source_file.to_string(),
                    TokenKind::Star,
                    self.position.0,
                    self.position.1,
                ))),

                // Handle potencial multi-line comment `/* */`
                '/' => {
                    if contents.peek() == Some(&'*') {
                        contents.next(); // Consume `*`
                        self.position.1 += 1;

                        while let Some(&c) = contents.peek() {
                            if c == '*' {
                                contents.next();
                                self.position.1 += 1;

                                if contents.peek() == Some(&'/') {
                                    contents.next();
                                    self.position.1 += 1;
                                    break;
                                }
                            } else if c == '\n' {
                                self.position.0 += 1;
                                self.position.1 = 1;
                            }
                        }

                        Ok(None)
                    } else {
                        Ok(Some(Token::static_tokenkind(
                            self.source_file.to_string(),
                            TokenKind::Slash,
                            self.position.0,
                            self.position.1,
                        )))
                    }
                },
    
                // Handle potential multi-character tokens (e.g., `==`, `!=`)
                '!' => {
                    if contents.peek() == Some(&'=') {
                        contents.next(); // Consume `=`
                        Ok(Some(Token::static_tokenkind(
                            self.source_file.to_string(),
                            TokenKind::BangEqual,
                            self.position.0,
                            self.position.1,
                        )))
                    } else {
                        Ok(Some(Token::static_tokenkind(
                            self.source_file.to_string(),
                            TokenKind::Bang,
                            self.position.0,
                            self.position.1,
                        )))
                    }
                }
                '=' => {
                    if contents.peek() == Some(&'=') {
                        contents.next(); // Consume `=`
                        Ok(Some(Token::static_tokenkind(
                            self.source_file.to_string(),
                            TokenKind::EqualEqual,
                            self.position.0,
                            self.position.1,
                        )))
                    } else {
                        Ok(Some(Token::static_tokenkind(
                            self.source_file.to_string(),
                            TokenKind::Equal,
                            self.position.0,
                            self.position.1,
                        )))
                    }
                }
    
                // Handle whitespace (advance position and ignore)
                ' ' | '\t' | '\r' => Ok(None),
    
                // Handle newlines (update line and reset column)
                '\n' => {
                    self.position.0 += 1;
                    self.position.1 = 1;
                    Ok(None)
                }
    
                // Handle identifiers and keywords
                c if c.is_alphabetic() || c == '_' => {
                    let mut identifier = String::new();
                    identifier.push(c);
    
                    while let Some(&next_char) = contents.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            identifier.push(next_char);
                            contents.next();
                        } else {
                            break;
                        }
                    }
    
                    if let Some(keyword_kind) = self.keywords.get(&identifier.borrow()) {
                        Ok(Some(Token::static_tokenkind(
                            self.source_file.to_string(),
                            (*keyword_kind).clone(), // clone
                            self.position.0,
                            self.position.1,
                        )))
                    } else { // if it is not a keyword its an identifier (name of either function or variable)
                        Ok(Token::dynamic_tokenkind(
                            self.source_file.to_string(),
                            TokenKind::Identifier,
                            identifier,
                            self.position.0,
                            self.position.1,
                        ))
                    }
                }
    
                // Handle numbers
                c if c.is_digit(10) => {
                    let mut number = String::new();
                    number.push(c);
    
                    while let Some(&next_char) = contents.peek() {
                        if next_char.is_digit(10) || next_char == '.' {
                            number.push(next_char);
                            contents.next();
                        } else {
                            break;
                        }
                    }
    
                    Ok(Token::dynamic_tokenkind(
                        self.source_file.to_string(),
                        TokenKind::Number,
                        number.parse().unwrap(),
                        self.position.0,
                        self.position.1,
                    ))
                }
    
                // Unknown character (error)
                _ => Err(format!("Unexpected character: '{}'.", char)),
            }
        } else {
            Err("Unexpected end of input.".to_string())
        }
    }

}