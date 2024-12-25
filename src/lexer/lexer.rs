use std::{borrow::Borrow, collections::HashMap, iter::Peekable};
use super::{tokens::{Token, TokenKind}, utils::StringStream};

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
    contents: StringStream,
    source: String,
    keywords: HashMap<&'static str, TokenKind>,
    position: (usize, usize),     // line, column //TODO
    current_position: usize
}

/*
    position needs to be done right, because it doesnt arrange well the position, all of that is for error displaying on console
    must be done, position must have the last index of the character
*/

impl Lexer {
    pub fn new(source_filename: &'static str, source: String) -> Self {
        Lexer {
            source_filename,
            contents: StringStream::new(source.clone()), // clone!
            source,
            keywords: get_keywords_hashmap(),
            position: (1, 1),
            current_position: 0
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];
        let mut errors = vec![];
        while let Some(_) = self.contents.peek() {
            let token_result = self.scan_token();
            match token_result {
                Ok(Some(token)) => {
                    tokens.push(token);
                }
                Ok(None) => {
                    continue; // No token found, just continue
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }
    
        // Add EOF token to the tokens vector
        tokens.push(Token::eof(
            String::from(self.source_filename),
            self.position.0,
            self.position.1 + 1,
        ));
    
        // If there were errors, collect them and return as a single error string
        if !errors.is_empty() {
            let joined = errors.join("\n");
            return Err(joined);
        }
    
        Ok(tokens)
    }
    
    fn scan_token(&mut self) -> Result<Option<Token>, String> {
        
        if let Some(char) = self.contents_next() {

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
                    if self.contents.peek() == Some(&'*') {
                        // Multi-line comment: /* */
                        self.contents_next(); // Consume `*`

                        while let Some(&c) = self.contents.peek() {
                            if c == '*' && self.contents.peek_next() == Some(&'/') {

                                self.contents_next(); // Consumes `*`
                                self.contents_next(); // Consumes `/`
                                self.position.1 -= 1; // WHAT
                                break;

                            } else if c == '\n' {
                                self.position.0 += 1;
                                self.position.1 = 1;
                                self.contents.next();

                            } else {
                                self.contents_next();
                            }
                        }

                        Ok(None) // Return `None` because this is a comment
                    } else if self.contents.peek() == Some(&'/') {
                        // One-line comment: `//`
                        self.contents_next(); // Consume the first `/`

                        // Consume characters until the end of the line
                        while let Some(&c) = self.contents.peek() {
                            if c == '\n' {
                                self.contents_next(); // Consume the newline
                                self.position.0 += 1; // New line
                                self.position.1 = 1;  // Reset column position
                                break;
                            } else {
                                self.contents_next(); // Consume the current character
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
                    if self.contents.peek() == Some(&'=') {
                        self.contents_next(); // Consume `=`
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
                    if self.contents.peek() == Some(&'=') {
                        self.contents_next(); // Consume `=`
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
                    if self.contents.peek() == Some(&'=') {
                        self.contents_next(); // consume `=`
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
                    if self.contents.peek() == Some(&'=') {
                        self.contents_next(); // consume `=`
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
    
                    while let Some(&next_char) = self.contents.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            identifier.push(next_char);
                            self.contents_next();
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
    
                    while let Some(&next_char) = self.contents.peek() {
                        if next_char.is_digit(10) || next_char == '.' {
                            number.push(next_char);
                            self.contents_next();
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

    fn contents_next(&mut self) -> Option<char> {
        self.position.1 += 1;
        self.contents.next()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn contents_next() {
        let source = "some source".to_string();
        let mut lexer = Lexer::new("filename", source);

        lexer.contents_next();

        assert_eq!(lexer.position, (1, 2));
        assert_eq!(lexer.contents.peek(), Some(&'o'))
    }

    #[test]
    fn scan_let() {
        let source_vec = "let".to_string();
        let mut lexer = Lexer::new("filename", source_vec);

        let result = lexer.scan_token().unwrap();



        assert_eq!(result, Some(Token::static_tokenkind("filename".to_string(), TokenKind::Let, 1, 4).unwrap()))
    }

    #[test]
    fn scan_identifier() {
        let mut lexer = Lexer::new("filename", "x".to_string());

        let result = lexer.scan_token().unwrap();

        assert_eq!(result, Some(Token::dynamic_tokenkind("filename".to_string(), TokenKind::Identifier, "x".to_string(), 1, 2).unwrap()))
    }

    #[test]
    fn scan_parenthtesis_braces() {
        let source = "(){}".to_string();
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
        let source = ",.-+;*/?:".to_string();
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
        let source = "// valid comment".to_string();
        let mut lexer = Lexer::new("filename", source);

        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::eof("filename".to_string(), 1, 18)
        ];

        assert_eq!(tokens, token_test);
    }

    #[test]
    fn valid_empty_multi_line_comment() {
        let source = "/**/".to_string();
        let mut lexer = Lexer::new("filename", source);
        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::eof("filename".to_string(), 1, 5)
        ];

        assert_eq!(tokens, token_test);
        assert_eq!(lexer.contents.peek(), None);
    }

    #[test]
    fn valid_multi_line_comment_one_line() {
        let source = "/* hi */".to_string();
        let mut lexer = Lexer::new("filename", source);
        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::eof("filename".to_string(), 1, 9)
        ];

        assert_eq!(tokens, token_test);
        assert_eq!(lexer.contents.peek(), None);
    }

    #[test]
    fn valid_empty_multi_line_comment_one_line() {
        let source = "/*\n*/".to_string();
        let mut lexer = Lexer::new("filename", source);
        let tokens = lexer.tokenize().unwrap();

        let token_test = vec![
            Token::eof("filename".to_string(), 2, 3)
        ];

        assert_eq!(tokens, token_test);
        assert_eq!(lexer.contents.peek(), None);
    }

    // #[test]
    // fn operators_one_char() {
    //     let source = "> < ! =".to_string();
    //     let mut lexer = Lexer::new("filename", source);
    //     let tokens = lexer.tokenize().unwrap();

    //     let token_test = vec![
    //         Token::static_tokenkind("filename".to_string(), TokenKind::Greater, 1, 2).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::Less, 1, 4).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::Bang, 1, 6).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::Equal, 1, 8).unwrap(),
    //         Token::eof("filename".to_string(), 1, 9)
    //     ];

    //     assert_eq!(tokens, token_test);
    //     assert_eq!(lexer.contents.peek(), None);
    // }

    // #[test]
    // fn operators_two_char() {
    //     let source = ">= <= != ==".to_string();
    //     let mut lexer = Lexer::new("filename", source);
    //     let tokens = lexer.tokenize().unwrap();

    //     let token_test = vec![
    //         Token::static_tokenkind("filename".to_string(), TokenKind::GreaterEqual, 1, 3).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::LessEqual, 1, 6).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::BangEqual, 1, 9).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::EqualEqual, 1, 12).unwrap(),
    //         Token::eof("filename".to_string(), 1, 13)
    //     ];

    //     assert_eq!(tokens, token_test);
    //     assert_eq!(lexer.contents.peek(), None);
    // }

    // #[test]
    // fn operators_two_char_other_order() {
    //     let source = "<= >= != ==".to_string();
    //     let mut lexer = Lexer::new("filename", source);
    //     let tokens = lexer.tokenize().unwrap();

    //     let token_test = vec![
    //         Token::static_tokenkind("filename".to_string(), TokenKind::LessEqual, 1, 3).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::GreaterEqual, 1, 6).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::BangEqual, 1, 9).unwrap(),
    //         Token::static_tokenkind("filename".to_string(), TokenKind::EqualEqual, 1, 12).unwrap(),
    //         Token::eof("filename".to_string(), 1, 13)
    //     ];

    //     assert_eq!(tokens, token_test);
    //     assert_eq!(lexer.contents.peek(), None);
    //}
}