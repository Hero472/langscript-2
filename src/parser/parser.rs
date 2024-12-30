use std::vec;

use crate::{ast::expr::Expr, lexer::tokens::{Token, TokenKind}, stmt::stmt::Stmt};

use super::utils::{match_token, TokenStream};

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

    fn primary() -> Result<Expr, String> {
        todo!()
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        todo!()
    }

    fn expression(&mut self) -> Result<Expr,String> {
        //self.assignment()
        todo!()
    }

    fn unary(&mut self) -> Result<Expr, String> {
        todo!()
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
        );

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