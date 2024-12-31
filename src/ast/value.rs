use std::{collections::HashMap, fmt, rc::Rc};

use crate::lexer::tokens::{Token, TokenKind};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Callable {
        name: String,
        arity: usize,
        fun: Rc<dyn Fn(&Vec<Value>) -> Value> // this should add the environment for that
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String(\"{}\")", s),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Array(arr) => {
                write!(f, "Array([")?;
                for (i, value) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", value)?;
                }
                write!(f, "])")
            }
            Value::Object(obj) => {
                write!(f, "Object({{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {:?}", key, value)?;
                }
                write!(f, "}})")
            }
            Value::Callable { name, arity, .. } => write!(f, "Callable(name: {}, arity: {})", name, arity),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (
                Value::Callable { name: a_name, arity: a_arity, .. },
                Value::Callable { name: b_name, arity: b_arity, .. },
            ) => a_name == b_name && a_arity == b_arity,
            _ => false,
        }
    }
}

impl Value {
    pub fn apply_unary_op(&self, token: &Token) -> Result<Value, String> {
        match self {
            Value::Number(x) => match token.kind {
                TokenKind::Minus => Ok(Value::Number(-x)),
                TokenKind::Plus => Ok(Value::Number(*x)), // Unary plus (no-op)
                _ => Err(format!(
                    "Unsupported unary operator {:?} for Number at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                )),
            },
            Value::Boolean(b) => match token.kind {
                TokenKind::Bang => Ok(Value::Boolean(!b)), // Logical NOT
                _ => Err(format!(
                    "Unsupported unary operator {:?} for Boolean at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                )),
            },
            Value::Array(arr) => match token.kind {
                TokenKind::Bang => Ok(Value::Boolean(arr.is_empty())),
                _ => Err(format!(
                    "Unsupported unary operator {:?} for Array at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                ))
            }
            _ => Err(format!(
                "Unary operator {:?} is not supported for this Value type at line {}, column {}",
                token.kind, token.line_number, token.column_number
            )),
        }
    }

    pub fn apply_binary_op(&self, token: &Token, other: &Value) -> Result<Value, String> {
        match (self, other) {
            // Arithmetic operations for numbers
            (Value::Number(lhs), Value::Number(rhs)) => match token.kind {
                TokenKind::Plus => Ok(Value::Number(lhs + rhs)),
                TokenKind::Minus => Ok(Value::Number(lhs - rhs)),
                TokenKind::Star => Ok(Value::Number(lhs * rhs)),
                TokenKind::Slash => {
                    if *rhs == 0.0 {
                        Err(format!("Division by zero ocurred at line {}, column: {}", token.line_number, token.column_number))
                    } else {
                        Ok(Value::Number(lhs / rhs))
                    }
                }
                TokenKind::Greater => Ok(Value::Boolean(lhs > rhs)),
                TokenKind::GreaterEqual => Ok(Value::Boolean(lhs >= rhs)),
                TokenKind::Less => Ok(Value::Boolean(lhs < rhs)),
                TokenKind::LessEqual => Ok(Value::Boolean(lhs <= rhs)),
                TokenKind::EqualEqual => Ok(Value::Boolean(lhs == rhs)),
                TokenKind::BangEqual => Ok(Value::Boolean(lhs != rhs)),
                _ => Err(format!(
                    "Unsupported binary operator {:?} for Numbers at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                )),
            },

            // Logical operations for booleans
            (Value::Boolean(lhs), Value::Boolean(rhs)) => match token.kind {
                TokenKind::And => Ok(Value::Boolean(*lhs && *rhs)),
                TokenKind::Or => Ok(Value::Boolean(*lhs || *rhs)),
                _ => Err(format!(
                    "Unsupported binary operator {:?} for Booleans at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                )),
            },

            // String concatenation
            (Value::String(lhs), Value::String(rhs)) => match token.kind {
                TokenKind::Plus => Ok(Value::String(format!("{}{}", lhs, rhs))),
                _ => Err(format!(
                    "Unsupported binary operator {:?} for Strings at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                )),
            },

            // Array concatenation (WHAT??)
            (Value::Array(lhs), Value::Array(rhs)) => match token.kind {
                TokenKind::Plus => {
                    let mut new_array = lhs.clone();
                    new_array.extend(rhs.clone());
                    Ok(Value::Array(new_array))
                },
                _ => Err(format!(
                    "Unsupported binary operator {:?} for Arrays at line {}, column {}",
                    token.kind, token.line_number, token.column_number
                )),
            }

            // Unsupported types or mismatches
            _ => Err(format!(
                "Binary operator {:?} is not supported for the given Value types at line {}, column {}",
                token.kind, token.line_number, token.column_number
            )),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.kind {
            TokenKind::Number => {
                if let Some(Value::Number(number)) = token.literal {
                    Self::Number(number)
                } else {
                    panic!(
                        "Expected a Number literal, but got {:?} in line {} column {}",
                        token.literal, token.line_number, token.column_number
                    );
                }
            }
            TokenKind::String => {
                if let Some(Value::String(string)) = token.literal {
                    Self::String(string)
                } else {
                    panic!(
                        "Expected a String literal, but got {:?} in line {} column {}",
                        token.literal, token.line_number, token.column_number
                    );
                }
            }
            TokenKind::True => Self::Boolean(true),
            TokenKind::False => Self::Boolean(false),
            _ => panic!(
                "Could not create Value from {:?} in line {} column {}",
                token.lexeme, token.line_number, token.column_number
            ),
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        if let Value::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let Value::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        if let Value::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(
            TokenKind::Identifier,
            "identifier".to_string(),
            1,
            1,
            0,
            "main.rs".to_string(),
        );

        let token2 = Token::new(
            TokenKind::Identifier,
            "identifier".to_string(),
            1,
            1,
            0,
            "main.rs".to_string(),
        );

        assert_eq!(token1, token2, "Tokens should be equal");
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new(
            TokenKind::Number,
            "123".to_string(),
            2,
            5,
            3,
            "main.rs".to_string(),
        );

        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.lexeme, "123".to_string());
        assert_eq!(token.line_number, 2);
        assert_eq!(token.column_number, 5);
        assert!(token.literal.is_none());
        assert!(token.parent_context.is_none());
        assert!(token.typed_token.is_none());
        assert!(!token.is_mutable);
        assert!(token.access_specifier.is_none());
        assert!(token.annotations.is_none());
        assert_eq!(token.source_file, Some("main.rs".to_string()));
    }

    #[test]
    fn test_static_tokenkind() {
        let token = Token::static_tokenkind(
            "main.rs".to_string(),
            TokenKind::LeftParen,
            3,
            7,
        )
        .unwrap();

        assert_eq!(token.kind, TokenKind::LeftParen);
        assert_eq!(token.lexeme, "(");
        assert_eq!(token.line_number, 3);
        assert_eq!(token.column_number, 7);
    }

    #[test]
    fn test_dynamic_tokenkind_identifier() {
        let token = Token::dynamic_tokenkind(
            "main.rs".to_string(),
            TokenKind::Identifier,
            "variable_name".to_string(),
            4,
            2,
        )
        .unwrap();

        assert_eq!(token.kind, TokenKind::Identifier);
        assert_eq!(token.lexeme, "variable_name");
    }

    #[test]
    fn test_dynamic_tokenkind_number() {
        let token = Token::dynamic_tokenkind(
            "main.rs".to_string(),
            TokenKind::Number,
            "42.0".to_string(),
            5,
            10,
        )
        .unwrap();

        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.literal, Some(Value::Number(42.0)));
    }

    #[test]
    fn test_dynamic_tokenkind_boolean() {
        let token = Token::dynamic_tokenkind(
            "main.rs".to_string(),
            TokenKind::Boolean,
            "true".to_string(),
            6,
            15,
        )
        .unwrap();

        assert_eq!(token.kind, TokenKind::Boolean);
        assert_eq!(token.literal, Some(Value::Boolean(true)));
    }

    #[test]
    fn test_dynamic_tokenkind_array() {
        let token = Token::dynamic_tokenkind(
            "main.rs".to_string(),
            TokenKind::Array,
            "[]".to_string(),
            7,
            20,
        )
        .unwrap();

        assert_eq!(token.kind, TokenKind::Array);
        assert_eq!(token.literal, Some(Value::Array(vec![])));
    }

    #[test]
    fn test_dynamic_tokenkind_object() {
        let token = Token::dynamic_tokenkind(
            "main.rs".to_string(),
            TokenKind::Object,
            "{}".to_string(),
            8,
            25,
        )
        .unwrap();

        assert_eq!(token.kind, TokenKind::Object);
        assert_eq!(token.literal, Some(Value::Object(HashMap::new())));
    }

    #[test]
    fn test_eof_token() {
        let token = Token::eof("main.rs".to_string(), 10, 30);

        assert_eq!(token.kind, TokenKind::EOF);
        assert_eq!(token.lexeme, "End of File");
        assert_eq!(token.line_number, 10);
        assert_eq!(token.column_number, 30);
    }
}