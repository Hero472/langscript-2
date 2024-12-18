use std::collections::HashMap;

use super::value::Value;

#[derive(Clone, PartialEq, Debug)] // DONT USE IT AAA
pub enum TokenKind {
    // Symbols
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    QuestionMark, Colon,

    // Operators
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number, Boolean, Array, Object,

    // Keywords
    And, Class, Else, False, True, Fn, For,
    If, Null, Or, Print, Return, Super, This,
    Let, While, Enum,

    // Flow Control
    Break, Continue,

    // End of file
    EOF
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String, // could be &str but lifetimes lmao
    literal: Option<Value>,
    line_number: usize,
    column_number: usize,
    end_offset: usize, // offset to the end of the token
    parent_context: Option<String>, // for now a String
    typed_token: Option<TypedToken>,
    is_mutable: bool,
    access_specifier: Option<AccessSpecifier>,
    annotations: Option<Vec<String>>,
    source_file: Option<String>,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.lexeme == other.lexeme
            && self.literal == other.literal
            && self.line_number == other.line_number
            && self.column_number == other.column_number
            && self.end_offset == other.end_offset
            && self.parent_context == other.parent_context
            && self.typed_token == other.typed_token
            && self.is_mutable == other.is_mutable
            && self.access_specifier == other.access_specifier
            && self.annotations == other.annotations
            && self.source_file == other.source_file
    }
}

impl Token {
    pub fn new(
        kind: TokenKind,
        lexeme: String,
        line_number: usize,
        column_number: usize,
        end_offset: usize,
        source_file: String
    ) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            literal: None,
            line_number,
            column_number,
            end_offset,
            parent_context: None,
            typed_token: None,
            is_mutable: false,
            access_specifier: None,
            annotations: None,
            source_file: Some(source_file),
        }
    }

    pub fn static_tokenkind(
        source_file: String,
        kind: TokenKind,
        line_number: usize,
        column_number: usize,
    ) -> Result<Self, String> {
        match kind {
            // Symbols
            TokenKind::LeftParen | TokenKind::RightParen | TokenKind::LeftBrace | TokenKind::RightBrace
            | TokenKind::Comma | TokenKind::Dot | TokenKind::Minus | TokenKind::Plus
            | TokenKind::Semicolon | TokenKind::Slash | TokenKind::Star | TokenKind::QuestionMark
            | TokenKind::Colon => Ok(Self {
                lexeme: Self::read_lexeme(&kind).to_string(),
                kind,
                literal: None,
                line_number,
                column_number,
                end_offset: column_number + 1,
                parent_context: None,
                typed_token: None,
                is_mutable: false,
                access_specifier: None,
                annotations: None,
                source_file: Some(source_file),
            }),
    
            // Operators
            TokenKind::Bang | TokenKind::BangEqual | TokenKind::Equal | TokenKind::EqualEqual
            | TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual => {
                Ok(Self {
                    lexeme: Self::read_lexeme(&kind).to_string(),
                    kind,
                    literal: None,
                    line_number,
                    column_number,
                    end_offset: column_number + 1,
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            }
    
            // Keywords
            TokenKind::And | TokenKind::Class | TokenKind::Else | TokenKind::False | TokenKind::True
            | TokenKind::Fn | TokenKind::For | TokenKind::If | TokenKind::Null | TokenKind::Or
            | TokenKind::Print | TokenKind::Return | TokenKind::Super | TokenKind::This
            | TokenKind::Let | TokenKind::While | TokenKind::Enum => Ok(Self {
                lexeme: Self::read_lexeme(&kind).to_string(),
                kind,
                literal: None,
                line_number,
                column_number,
                end_offset: column_number + 1,
                parent_context: None,
                typed_token: None,
                is_mutable: false,
                access_specifier: None,
                annotations: None,
                source_file: Some(source_file),
            }),
    
            // Flow Control
            TokenKind::Break | TokenKind::Continue => Ok(Self {
                lexeme: Self::read_lexeme(&kind).to_string(),
                kind,
                literal: None,
                line_number,
                column_number,
                end_offset: column_number + 1,
                parent_context: None,
                typed_token: None,
                is_mutable: false,
                access_specifier: None,
                annotations: None,
                source_file: Some(source_file),
            }),
    
            // Unsupported or invalid token kinds
            _ => Err(format!(
                "Unsupported token kind '{:?}' encountered at line {} column {}",
                kind, line_number, column_number
            )),
        }
    }

    // this Fnctio actually may return Result<Option<Self>, String> but I dont know
    pub fn dynamic_tokenkind(
        source_file: String,
        kind: TokenKind,
        lexeme: String,
        line_number: usize,
        column_number: usize,
    ) -> Result<Self, String> { // Token and token Offset
        match kind {
            TokenKind::Identifier => {
                // Handle identifiers
                Ok(Self {
                    kind,
                    lexeme: lexeme.to_string(),
                    literal: None, // Identifiers usually don't have a literal value.
                    line_number,
                    column_number,
                    end_offset: column_number + lexeme.len(),
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            },
            TokenKind::String => {
                Ok(Self {
                    lexeme: lexeme.to_string(),
                    kind,
                    literal: Some(Value::String(lexeme.to_string())), // maybe it modifies so thats why its String
                    line_number,
                    column_number,
                    end_offset: column_number + lexeme.len(),
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            },
            TokenKind::Number => {

                let num = lexeme.parse::<f64>().unwrap();

                Ok(Self {
                    lexeme: lexeme.to_string(),
                    kind,
                    literal: Some(Value::Number(num)),
                    line_number,
                    column_number,
                    end_offset: column_number + lexeme.len(),
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            },
            TokenKind::Boolean => {

                let bool = lexeme.parse::<bool>().unwrap();

                Ok(Self {
                    lexeme: lexeme.to_string(),
                    kind,
                    literal: Some(Value::Boolean(bool)),
                    line_number,
                    column_number,
                    end_offset: column_number + lexeme.len(),
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            },
            TokenKind::Array => { // TODO
                Ok(Self {
                    lexeme: lexeme.to_string(),
                    kind,
                    literal: Some(Value::Array(vec![])),
                    line_number,
                    column_number,
                    end_offset: column_number + lexeme.len(),
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            },
            TokenKind::Object => { // TODO
                Ok(Self {
                    lexeme: lexeme.to_string(),
                    kind,
                    literal: Some(Value::Object(HashMap::new())),
                    line_number,
                    column_number,
                    end_offset: column_number + lexeme.len(),
                    parent_context: None,
                    typed_token: None,
                    is_mutable: false,
                    access_specifier: None,
                    annotations: None,
                    source_file: Some(source_file),
                })
            },
            _ => return Err(format!("An error ocurred in the Tokenization process in {} \nline: {}\ncolumn: {}", lexeme, line_number, column_number))
        }
    }

    pub fn eof(
        source_file: String,
        line_number: usize,
        column_number: usize
    ) -> Self {
        Self {
            kind: TokenKind::EOF,
            lexeme: "End of File".to_string(),
            literal: None,
            line_number,
            column_number,
            end_offset: column_number + 1,
            parent_context: None,
            typed_token: None,
            is_mutable: false,
            access_specifier: None,
            annotations: None,
            source_file: Some(source_file),
        }
    }

    fn read_lexeme(kind: &TokenKind) -> &'static str {
        match kind {
            // Symbols
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Minus => "-",
            TokenKind::Plus => "+",
            TokenKind::Semicolon => ";",
            TokenKind::Slash => "/",
            TokenKind::Star => "*",
            TokenKind::QuestionMark => "?",
            TokenKind::Colon => ":",

            // Operators (still symbols I guess)
            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",

            // Keywords
            TokenKind::And => "and",
            TokenKind::Class => "class",
            TokenKind::Else => "else",
            TokenKind::False => "false",
            TokenKind::True => "true",
            TokenKind::Fn => "Fn",
            TokenKind::For => "for",
            TokenKind::If => "if",
            TokenKind::Null => "null",
            TokenKind::Or => "or",
            TokenKind::Print => "print",
            TokenKind::Return => "return",
            TokenKind::Super => "super",
            TokenKind::This => "this",
            TokenKind::Let => "let",
            TokenKind::While => "while",
            TokenKind::Enum => "enum",

            // Flow Control
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",

            _ => ""

        }
    }
}

//TODO
#[derive(Debug, PartialEq)]
enum DataType {
    Number,
    String,
    Boolean,
    Array(Box<DataType>),
    Object,
    Fnction(Vec<DataType>, Box<DataType>), // arguments, return values
    Enum {
        name: String,                             // Enum name
        variants: HashMap<String, DataType>, // Variant name and associated types
    },
    Error(String),
}

//TODO
#[derive(Debug, PartialEq)]
pub struct TypedToken {
    data_type: DataType, // Type of the token
    value: Option<Value>, // Value of the token, if any
}
//TODO
#[derive(Debug, PartialEq)]
enum AccessSpecifier {
    Public,
    Private
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(
            TokenKind::Identifier,
            "example".to_string(),
            1,
            5,
            12,
            "test.rs".to_string(),
        );
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.lexeme, "example");
        assert_eq!(token.line_number, 1);
        assert_eq!(token.column_number, 5);
        assert_eq!(token.end_offset, 12);
        assert_eq!(token.source_file, Some("test.rs".to_string()));
    }

    #[test]
    fn test_token_static_tokenkind() {
        let token = Token::static_tokenkind(
            "test.rs".to_string(),
            TokenKind::LeftParen,
            2,
            10,
        );
        let token = token.unwrap(); // Unwrap the token only once

        assert_eq!(token.kind, TokenKind::LeftParen);
        assert_eq!(token.lexeme, "(");
        assert_eq!(token.line_number, 2);
        assert_eq!(token.column_number, 10);
        assert_eq!(token.end_offset, 11);
    }

    #[test]
    fn test_token_dynamic_tokenkind_identifier() {
        let token = Token::dynamic_tokenkind(
            "test.rs".to_string(),
            TokenKind::Identifier,
            "my_var".to_string(),
            3,
            15,
        );
        assert!(token.is_ok());
        let token = token.unwrap();
        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.lexeme, "my_var");
        assert_eq!(token.line_number, 3);
        assert_eq!(token.column_number, 15);
        assert_eq!(token.end_offset, 21);
    }

    #[test]
    fn test_token_dynamic_tokenkind_string() {
        let token = Token::dynamic_tokenkind(
            "test.rs".to_string(),
            TokenKind::String,
            "hello world".to_string(),
            4,
            20,
        );
        assert!(token.is_ok());
        let token = token.unwrap();
        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.lexeme, "hello world");
        assert_eq!(token.literal, Some(Value::String("hello world".to_string())));
        assert_eq!(token.line_number, 4);
        assert_eq!(token.column_number, 20);
    }

    #[test]
    fn test_token_dynamic_tokenkind_number() {
        let token = Token::dynamic_tokenkind(
            "test.rs".to_string(),
            TokenKind::Number,
            "42.5".to_string(),
            5,
            25,
        );
        assert!(token.is_ok());
        let token = token.unwrap();
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.lexeme, "42.5");
        assert_eq!(token.literal, Some(Value::Number(42.5)));
        assert_eq!(token.line_number, 5);
        assert_eq!(token.column_number, 25);
    }

    #[test]
    fn test_token_eof() {
        let token = Token::eof("test.rs".to_string(), 6, 30);
        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.lexeme, "End of File");
        assert_eq!(token.line_number, 6);
        assert_eq!(token.column_number, 30);
    }

    #[test]
    fn test_access_specifier() {
        assert_eq!(AccessSpecifier::Public, AccessSpecifier::Public);
        assert_eq!(AccessSpecifier::Private, AccessSpecifier::Private);
    }

    #[test]
    fn test_data_type_enum() {
        let mut variants = HashMap::new();
        variants.insert("Red".to_string(), DataType::String);
        variants.insert("Green".to_string(), DataType::String);

        let enum_type = DataType::Enum {
            name: "Color".to_string(),
            variants,
        };

        if let DataType::Enum { name, variants } = enum_type {
            assert_eq!(name, "Color");
            assert!(variants.contains_key("Red"));
            assert!(variants.contains_key("Green"));
        } else {
            panic!("Expected Enum type");
        }
    }

    #[test]
    fn test_typed_token() {
        let typed_token = TypedToken {
            data_type: DataType::Number,
            value: Some(Value::Number(10.0)),
        };

        assert_eq!(typed_token.data_type, DataType::Number);
        assert_eq!(typed_token.value, Some(Value::Number(10.0)));
    }
}

