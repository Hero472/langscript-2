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
    And, Class, Else, False, True, Fun, For,
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
    ) -> Self {
        Self {
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
        }
    }

    // this functio actually may return Result<Option<Self>, String> but I dont know
    pub fn dynamic_tokenkind(
        source_file: String,
        kind: TokenKind,
        lexeme: String,
        line_number: usize,
        column_number: usize,
    ) -> Option<Self> { // Token and token Offset
        match kind {
            TokenKind::Identifier => {
                // Handle identifiers
                Some(Self {
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
                Some(Self {
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

                Some(Self {
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

                Some(Self {
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
                Some(Self {
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
                Some(Self {
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
            _ => None
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
            TokenKind::Fun => "fun",
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
    Function(Vec<DataType>, Box<DataType>), // arguments, return values
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
