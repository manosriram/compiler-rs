use core::panic;

use crate::tokenizer::{Token, TokenType};


pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String)
}

pub enum Op {
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY
}

pub enum Expr {
    BinOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },

    Literal(Literal),

    UnaryOp {
        op: Op,
        expr: Box<Expr>,
    },

    Ident {
        name: String
    },

    None {}
}

pub enum Statement {
    Let {
        name: String,
        value: Box<Expr>
    },
    Redef {
        name: String,
        value: Box<Expr>
    },
    Expr(Expr),
    None{}
}

pub struct Ast {
    statements: Vec<Statement>,
    tokens: Vec<Token>,
    current_token_idx: usize
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Ast {
        Ast{
            statements: vec![],
            tokens: tokens,
            current_token_idx: 0
        }
    }

    fn peek(&self) -> Option<Token> {
        if self.tokens.len() > self.current_token_idx {
            self.tokens.get(self.current_token_idx + 1).cloned()
        } else {
            None
        }
    }

    fn get_current_token(&self) -> Option<Token> {
        self.tokens.get(self.current_token_idx).cloned()
    }

    fn advance(&mut self) {
        self.current_token_idx += 1;
    }

    fn is(&mut self, typ: TokenType) -> bool {
        if matches!(self.get_current_token().unwrap().typ, typ) {
            self.advance();
            return true;
        }
        false
    }

    fn parse_stmt(&mut self) -> Result<Statement, String> {
        match self.get_current_token().unwrap().typ {
            // TODO: handle other tokens
            TokenType::LET => {
                self.advance();
                let name = self.get_current_token().unwrap();
                self.advance();
                self.is(TokenType::EQUALS);
                let val = self.expr();
                Ok(Statement::Let{
                    name: name.value.unwrap(),
                    value: Box::from(val)
                })
            },
            _ => {Err(String::from(""))}
        }
    }

    fn expr(&mut self) -> Expr {
        let l = self.term();
        self.advance();
        while matches!(self.get_current_token().unwrap().typ, TokenType::PLUS | TokenType::MINUS) {
            let tok = self.get_current_token().unwrap().typ;
            self.advance();
            let r = self.term();
            return match tok {
                TokenType::PLUS => Expr::BinOp { left: Box::from(l), op: Op::PLUS, right: Box::from(r) },
                TokenType::MINUS => Expr::BinOp { left: Box::from(l), op: Op::MINUS, right: Box::from(r) },
                _ => Expr::None{}
            };
        }
        l
    }

    fn term(&mut self) -> Expr {
        let l = self.factor();
        self.advance();
        while matches!(self.get_current_token().unwrap().typ, TokenType::MULTIPLY | TokenType::DIVIDE) {
            let tok = self.get_current_token().unwrap().typ;
            self.advance();
            let r = self.term();
            return match tok {
                TokenType::MULTIPLY => Expr::BinOp { left: Box::from(l), op: Op::MULTIPLY, right: Box::from(r) },
                TokenType::DIVIDE => Expr::BinOp { left: Box::from(l), op: Op::DIVIDE, right: Box::from(r) },
                _ => Expr::None{}
            };
        }
        l
    }

    fn factor(&mut self) -> Expr {
        let z = self.get_current_token().unwrap();
        match z.typ {
            // TODO: handle other tokens
            TokenType::IDENT => Expr::Ident { name: (z.value.unwrap()) },
            TokenType::LITERAL => {
                return Expr::Literal(Literal::String(z.value.unwrap()))
            },
            TokenType::LPAREN => {
                self.advance();
                let e = self.expr();
                self.advance();
                e
            },
            _ => panic!("Parse error")
        }
    }



    pub fn build(&mut self) {
        let stmt = self.parse_stmt();
        match stmt {
            Ok(v) => match v {
                // TODO: handle other tokens
                Statement::Let { name, value } => {
                    println!("got var {}", name);
                },
                _ => {
                }
            },
            Err(e) => {
                panic!("Error building ast {}", e);
            }
        };
    }
}
