use std::vec;

use crate::{ast::{expr::Expr, value::Value}, lexer::tokens::{Token, TokenKind}, stmt::stmt::Stmt};

use super::utils::{match_token, match_tokens, TokenStream};

pub struct Parser {
    pub(crate) tokens: TokenStream,
    current: usize,
    length: usize
}

// for future develop: in functions or a method in a class
#[derive(Debug)]
enum FunctionKind {
    Function,
    Method
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            length: tokens.len(), // isnt in order because of borrow checker
            tokens: TokenStream::new(tokens),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        
        let mut stmts = vec![];
        let mut errors = vec![];

        while !self.tokens.is_eof() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errors.push(msg);
                    self.synchronize();
                },
            }
        }
        
        if errors.len() == 0 {
            Ok(stmts)
        } else {
            Err(errors.join("\n"))
        }

    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if match_token(self, &TokenKind::Fn) {
            self.function_declaration(&FunctionKind::Function)
        } else {
            self.statement()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.tokens.peek();
        let result;

        match &token.clone().unwrap().kind {
            TokenKind::LeftParen => {
                self.tokens.next();
                let expr = self.expression()?;
                self.tokens.consume(TokenKind::RightParen,
                    format!(
                        "Expected ')' in line {} column {}",
                        self.tokens.peek().unwrap().line_number,
                        self.tokens.peek().unwrap().column_number
                    ).as_str()
                );
                result = Expr::Grouping { expression: Box::new(expr) }
            },
            TokenKind::False | TokenKind::True | TokenKind::Number | TokenKind::String => {
                self.tokens.next(); // why the fuck
                result = Expr::Literal { value: Value::from_token(token.unwrap().clone()) }
            },
            TokenKind::Identifier => {
                todo!() // for variables
            },
            TokenKind::Fn => {
                todo!() //anonymous functions
            },
            ttype => return Err(format!("Expected expression, last token read was {:?} in line {} column {}",
                ttype, 
                token.unwrap().line_number,
                token.unwrap().column_number
            ))
        }
        Ok(result)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if match_token(self, &TokenKind::LeftBrace) {
            self.block_statement()
        } else {
            self.expression_stmt()
        }
    }

    fn expression_stmt(&mut self) -> Result<Stmt,String> {
        let expression = self.expression()?;

        self.tokens.consume(TokenKind::Semicolon, 
            format!("Expected ';' after block statement in line {} column {}",
            self.tokens.peek().unwrap().line_number,
            self.tokens.peek().unwrap().column_number
        ).as_str()
        )?;

        Ok(Stmt::Expression { expression })
    }

    fn expression(&mut self) -> Result<Expr,String> {
        self.unary()
    }

    fn unary(&mut self) -> Result<Expr, String> {
        
        if match_tokens(self, &[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.tokens.previous().unwrap();
            let right = self.unary()?;

            Ok(Expr::Unary { operator, right: Box::new(right) })
        } else {
            self.call()
        }

    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if match_token(self, &TokenKind::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];

        if !self.check(TokenKind::RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);

                if arguments.len() >= 255 {
                    let location = self.tokens.peek().unwrap().line_number;
                    return Err(format!("Function cant have more than 255 arguments, in line {}", location))
                }

                if !match_token(self, &TokenKind::Comma) {
                    break;
                }
            }
        }

        let paren = self.tokens.consume(TokenKind::RightParen,
            format!("Expected ')' after arguments in line {} column {}",
            self.tokens.peek().unwrap().line_number,
            self.tokens.peek().unwrap().column_number
        ).as_str())?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn function_declaration(&mut self, fn_kind: &FunctionKind) -> Result<Stmt, String> {

        let name = self.tokens.consume(TokenKind::Identifier,
        format!("Expected {fn_kind:?} name in line {} column {}", 
                self.tokens.peek().unwrap().line_number,
                self.tokens.peek().unwrap().column_number
            ).as_str()
        )?;

        self.tokens.consume(TokenKind::LeftParen,
            format!("Expected '(' after {fn_kind:?} named {:?} in line {} column {}",
                name.lexeme,
                self.tokens.peek().unwrap().line_number,
                self.tokens.peek().unwrap().column_number
            ).as_str()
        )?;

        let mut params = vec![];

        if !self.check(TokenKind::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(format!("More than 255 parameters in function {:?}", name))
                }
    
                let param = self.tokens
                    .consume(TokenKind::Identifier, "Expected parameter name")?;
    
                params.push(param);
    
                if !match_token(self, &TokenKind::Comma) {
                    break;
                }
            }
        }

        self.tokens.consume(TokenKind::RightParen, format!("Expected ')' after parameters in line {} column {}",
            self.tokens.peek().unwrap().line_number,
            self.tokens.peek().unwrap().column_number,
        ).as_str())?;

        self.tokens.consume(TokenKind::RightParen, format!("Expected '{{' after parameters in line {} column {}",
            self.tokens.peek().unwrap().line_number,
            self.tokens.peek().unwrap().column_number,
        ).as_str())?;

        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that wasnt a block")
        };

        Ok(Stmt::Function { 
            name,
            params,
            body 
        })
    }

    fn function_expression(&mut self) {
        
    }

    fn check(&mut self, typ: TokenKind) -> bool {
        self.tokens.peek().unwrap().kind == typ
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements: Vec<Box<Stmt>> = vec![];

        while !self.check(TokenKind::RightBrace) && !self.tokens.is_eof() {
            let decl: Stmt = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.tokens.consume(TokenKind::RightBrace, "Expected '}' after a block")?;
        Ok(Stmt::Block { statements })
    }

    // this what it does it to look for the next keyword after and error to keep looking normally from there
    fn synchronize(&mut self) {
        self.tokens.next();

        while !self.tokens.is_eof() {

            if let Some(token) = self.tokens.previous() {
                if token.kind == TokenKind::Semicolon { return; }

                match token.kind {
                    TokenKind::Class | TokenKind::Fn | TokenKind::Let | TokenKind::If |
                    TokenKind::While | TokenKind::For | TokenKind::Return | TokenKind::Enum |
                    TokenKind::Match
                    => return,
                    _ => ()
                }
                self.tokens.next();
            }

        }
    }

}