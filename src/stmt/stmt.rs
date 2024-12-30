use crate::{ast::expr::Expr, lexer::tokens::Token};

pub enum Stmt {
    Expression {
        expression: Expr
    },
    Let {
        name: Token,
        initializer: Expr
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>
    },
    Block {
        statements: Vec<Box<Stmt>>
    }
}