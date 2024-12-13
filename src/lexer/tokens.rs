pub enum TokenKind {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    QuestionMark, Colon,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, True, Fun, For,
    If, Null, Or, Print, Return, Super, This,
    Let, While,

    Break, Continue,

    EOF
}

pub enum Value {
    Number(f64),
    String(String),
}

pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<Value>,
    line_number: usize,
    column_number: usize
}

impl Token {
    
}