use std::{borrow::Borrow, collections::HashMap, iter::Peekable};

use crate::lexer::utils::read_file;

use super::tokens::{Token, TokenKind};

fn get_keywords_hashmap() -> HashMap<&'static str, TokenKind> {
    HashMap::from([
        ("and", TokenKind::And),
        ("class", TokenKind::Class),
        ("else", TokenKind::Else),
        ("false", TokenKind::False),
        ("true", TokenKind::True),
        ("fun", TokenKind::Fn),
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

#[derive(Debug)]
pub struct Lexer {
    source_filename: &'static str,
    source_char: Vec<char>,
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenKind>,
    position: (usize, usize),     // line, column
}

impl Lexer {
    pub fn new(source_filename: &'static str, source_char: Vec<char>) -> Self {
        Lexer {
            source_filename,
            source_char,
            tokens: vec![],
            keywords: get_keywords_hashmap(),
            position: (1, 1)
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {

        let mut contents = self.source_char
            .clone() // This clone cost a lot, help
            .into_iter()
            .peekable();

        let mut tokens= vec![];
        let mut errors = vec![];
        while let Some(_) = contents.peek() {
            let token_result: Result<Option<Token>, String> = self.scan_token(&mut contents);
            println!("token_result: {:?}", token_result);
            match token_result {
                Ok(Some(token)) => {
                    tokens.push(token);
                }
                Ok(None) => {
                    continue;
                    // this is good behaviour I guess (just and only if it is well implemented)
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }

        tokens.push(Token::eof(
            String::from(self.source_filename),
            self.position.0,
            self.position.1 + 1
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

        Ok(tokens)
    }
    
    fn scan_token<I>(&mut self, contents: &mut Peekable<I>) -> Result<Option<Token>, String>
    where
        I: Iterator<Item = char>
    {
        
        if let Some(char) = self.contents_next(contents) {

            match char {
                // Handle single-character static_tokenkinds
                '(' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::LeftParen,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                ')' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::RightParen,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '{' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::LeftBrace,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '}' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::RightBrace,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                ',' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Comma,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '.' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Dot,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                ';' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Semicolon,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '?' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::QuestionMark,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                ':' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Colon,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '+' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Plus,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '-' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Minus,
                    self.position.0,
                    self.position.1,
                ).map(Some),
                '*' => Token::static_tokenkind(
                    self.source_filename.to_string(),
                    TokenKind::Star,
                    self.position.0,
                    self.position.1,
                ).map(Some),

                // Handle potential multi-line and one-line comments
                '/' => {
                    if contents.peek() == Some(&'*') {
                        // Multi-line comment: /* */
                        self.contents_next(contents); // Consume `*`

                        while let Some(&c) = contents.peek() {
                            if c == '*' {
                                self.contents_next(contents);

                                if contents.peek() == Some(&'/') {
                                    self.contents_next(contents); // Consume `>`
                                    break; // End of multi-line comment
                                }
                            } else if c == '\n' {
                                self.position.0 += 1; // New line
                                self.position.1 = 1;   // Reset column position
                            }
                        }

                        Ok(None) // Return `None` because this is a comment
                    } else if contents.peek() == Some(&'/') {
                        // One-line comment: `//`
                        self.contents_next(contents); // Consume the first `/`

                        // Consume characters until the end of the line
                        while let Some(&c) = contents.peek() {
                            if c == '\n' {
                                self.contents_next(contents); // Consume the newline
                                self.position.0 += 1; // New line
                                self.position.1 = 1;  // Reset column position
                                break;
                            } else {
                                self.contents_next(contents); // Consume the current character
                            }
                        }

                        Ok(None) // Return `None` because this is a comment
                    } else {
                        // Handle the `/` token (not part of a comment)
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::Slash,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    }
                },
    
                // Handle potential multi-character tokens (e.g., `==`, `!=`)
                '!' => {
                    if contents.peek() == Some(&'=') {
                        self.contents_next(contents); // Consume `=`
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::BangEqual,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    } else {
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::Bang,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    }
                }
                '=' => {
                    if contents.peek() == Some(&'=') {
                        self.contents_next(contents); // Consume `=`
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::EqualEqual,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    } else {
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::Equal,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    }
                }

                '>' => {
                    if contents.peek() == Some(&'=') {
                        self.contents_next(contents); // consume `=`
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::GreaterEqual,
                            self.position.0,
                            self.position.1
                        ).map(Some)
                    } else {
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::Greater,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    }
                },

                '<' => {
                    if contents.peek() == Some(&'=') {
                        self.contents_next(contents); // consume `=`
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::LessEqual,
                            self.position.0,
                            self.position.1
                        ).map(Some)
                    } else {
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::Less,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    }
                }
    
                // Handle whitespace (advance position and ignore)
                ' ' => {
                    self.position.1 += 1;
                    Ok(None)
                }
                
                '\t' => {
                    self.position.1 += 4; // tab = 4
                    Ok(None)
                }

                '\r' => Ok(None),
    
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
                            self.contents_next(contents);
                        } else {
                            break;
                        }
                    }
    
                    if let Some(keyword_kind) = self.keywords.get(&identifier.borrow()) {
                        Token::static_tokenkind(
                            self.source_filename.to_string(),
                            (*keyword_kind).clone(), // clone
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    } else { // if it is not a keyword its an identifier (name of either function or variable)
                        Token::dynamic_tokenkind(
                            self.source_filename.to_string(),
                            TokenKind::Identifier,
                            identifier,
                            self.position.0,
                            self.position.1,
                        ).map(Some)
                    }
                }
    
                // Handle numbers
                c if c.is_digit(10) => {
                    let mut number = String::new();
                    number.push(c);
    
                    while let Some(&next_char) = contents.peek() {
                        if next_char.is_digit(10) || next_char == '.' {
                            number.push(next_char);
                            self.contents_next(contents);
                        } else {
                            break;
                        }
                    }
    
                    Token::dynamic_tokenkind(
                        self.source_filename.to_string(),
                        TokenKind::Number,
                        number,
                        self.position.0,
                        self.position.1,
                    ).map(Some)
                }
    
                // Unknown character (error)
                _ => Err(format!("Unexpected character: '{}'.", char)),
            }
        } else {
            Err("Unexpected end of input.".to_string())
        }
    }


    fn contents_next<I>(&mut self, contents: &mut Peekable<I>) -> Option<char>
    where I: Iterator<Item = char>
    {
        self.position.1 += 1;
        contents.next()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn contents_next() {
        let mut source = "some source".chars().into_iter().peekable();
        let source_copy = "some source".chars().into_iter().collect::<Vec<char>>();
        let mut lexer = Lexer::new("filename", source_copy);

        lexer.contents_next(&mut source);

        assert_eq!(lexer.position, (1, 2))
    }

    #[test]
    fn scan_let() {
        let mut source = "let".chars().into_iter().peekable();
        let source_vec = "let".chars().into_iter().collect::<Vec<char>>();
        let mut lexer = Lexer::new("filename", source_vec);

        let result = lexer.scan_token(&mut source).unwrap();



        assert_eq!(result, Some(Token::static_tokenkind("filename".to_string(), TokenKind::Let, 1, 4).unwrap()))
    }

    #[test]
    fn scan_identifier() {
        let mut source = "x".chars().into_iter().peekable();
        let source_copy = "x";
        let mut lexer = Lexer::new(source_copy, vec!['x']);

        let result = lexer.scan_token(&mut source).unwrap();

        assert_eq!(result, Some(Token::dynamic_tokenkind(source_copy.to_string(), TokenKind::Identifier, "x".to_string(), 1, 2).unwrap()))
    }

    #[test]
    fn scan_parenthtesis_braces() {
        let source = "(){}".chars().into_iter().collect::<Vec<char>>();
        let mut lexer = Lexer::new("filename", source);

        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::static_tokenkind("filename".to_string(), TokenKind::LeftParen, 1, 2).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::RightParen, 1, 3).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::LeftBrace, 1, 4).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::RightBrace, 1, 5).unwrap(),
            Token::eof("filename".to_string(), 1, 6)
        ];

        assert_eq!(tokens, token_test);
    }

    #[test]
    fn scan_symbols_and_operators() {
        let source = ",.-+;*/?:".chars().into_iter().collect::<Vec<char>>();
        let mut lexer = Lexer::new("filename", source);

        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::static_tokenkind("filename".to_string(), TokenKind::Comma, 1, 2).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Dot, 1, 3).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Minus, 1, 4).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Plus, 1, 5).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Semicolon, 1, 6).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Star, 1, 7).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Slash, 1, 8).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::QuestionMark, 1, 9).unwrap(),
            Token::static_tokenkind("filename".to_string(), TokenKind::Colon, 1, 10).unwrap(),
            Token::eof("filename".to_string(), 1, 11)
        ];

        assert_eq!(tokens, token_test);
    }

    #[test]
    fn valid_one_line_comment() {
        let source = "// valid comment".chars().into_iter().collect::<Vec<char>>();
        let mut lexer = Lexer::new("filename", source);

        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::eof("filename".to_string(), 1, 18)
        ];

        assert_eq!(tokens, token_test);
    }

    // #[test]
    // fn valid_multi_line_comment() {
    //     let source = "/* valid multi line\ncomment*/".chars().into_iter().collect::<Vec<char>>();
    //     let mut lexer = Lexer::new("filename", source);

    //     let tokens = lexer.tokenize().unwrap();

    //     let token_test = vec![
    //         Token::eof("filename".to_string(), 2, 9)
    //     ];

    //     assert_eq!(tokens, token_test);
    // }
}