use crate::lexer::tokens::Token;

use super::value::Value;

pub enum Expr {
    Literal {
        value: Value,
    },
    Grouping {
        expression: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    }
}

impl Expr {

    pub fn evaluate(&self) -> Result<Value, String> {
        match self {
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Binary { left, operator, right } => {
                let left_value = left.evaluate()?;
                let right_value = right.evaluate()?;
                left_value.apply_binary_op(operator, &right_value)
            },
            Expr::Unary { operator, right } => {
                let right_value = right.evaluate()?;
                right_value.apply_unary_op(operator)
            },
            Expr::Call { callee, paren, arguments } => {
                let callable = (*callee).evaluate()?;

                match callable {
                    Value::Callable { 
                        name,
                        arity,
                        fun 
                    } => {

                        if arguments.len() != arity {
                            return Err(format!(
                                "Callable {} expected {} arguments but got {}",
                                name,
                                arity,
                                arguments.len()
                            ));
                        }

                        let mut arguments_values = vec![];

                        for arg in arguments {
                            let val = arg.evaluate()?;
                            arguments_values.push(val);
                        }

                        Ok(fun(&arguments_values))
                    },
                    other => Err("is not callable".to_string())
                }
            },
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokens::{Token, TokenKind};

    #[test]
    fn test_literal_evaluation() {
        let expr = Expr::Literal {
            value: Value::Number(42.0),
        };
        let result = expr.evaluate();
        assert_eq!(result, Ok(Value::Number(42.0)));
    }

    #[test]
    fn test_grouping_evaluation() {
        let expr = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Value::Number(10.0),
            }),
        };
        let result = expr.evaluate();
        assert_eq!(result, Ok(Value::Number(10.0)));
    }

    #[test]
    fn test_binary_evaluation_addition() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Value::Number(5.0),
            }),
            operator: Token::static_tokenkind(
                "filename".to_string(),
                TokenKind::Plus,
                1,
                1).unwrap(),
            right: Box::new(Expr::Literal {
                value: Value::Number(3.0),
            }),
        };
        let result = expr.evaluate();
        assert_eq!(result, Ok(Value::Number(8.0)));
    }

    #[test]
    fn test_unary_evaluation_negation() {
        let expr = Expr::Unary {
            operator: Token::static_tokenkind(
                "filename".to_string(),
                TokenKind::Minus,
                1,
                1).unwrap(),
            right: Box::new(Expr::Literal {
                value: Value::Number(5.0),
            }),
        };
        let result = expr.evaluate();
        assert_eq!(result, Ok(Value::Number(-5.0)));
    }

    #[test]
    fn test_binary_evaluation_comparison() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Value::Number(10.0),
            }),
            operator: Token::static_tokenkind(
                "filename".to_string(),
                TokenKind::Greater,
                1,
                1).unwrap(),
            right: Box::new(Expr::Literal {
                value: Value::Number(5.0),
            }),
        };
        let result = expr.evaluate();
        assert_eq!(result, Ok(Value::Boolean(true)));
    }

    #[test]
    fn test_binary_evaluation_division_by_zero() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Value::Number(10.0),
            }),
            operator: Token::static_tokenkind(
                "filename".to_string(),
                TokenKind::Slash,
                1,
                1).unwrap(),
            right: Box::new(Expr::Literal {
                value: Value::Number(0.0),
            }),
        };
        let result = expr.evaluate();
        assert!(result.is_err());
    }
}