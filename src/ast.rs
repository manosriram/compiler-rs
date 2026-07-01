use core::panic;

use crate::tokenizer::{Token, TokenType};

pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

pub enum Op {
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY,
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
        name: String,
    },

    None {},
}

pub enum Statement {
    Let { name: String, value: Box<Expr> },
    Redef { name: String, value: Box<Expr> },
    Expr(Expr),
    None {},
}

pub struct Ast {
    statements: Vec<Statement>,
    tokens: Vec<Token>,
    current_token_idx: usize,
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Ast {
        Ast {
            statements: vec![],
            tokens: tokens,
            current_token_idx: 0,
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
        // println!("ty = {} {}", typ ,self.get_current_token().unwrap().value.unwrap().to_string());
        if matches!(self.get_current_token().unwrap().typ, typ) {
            self.advance();
            return true;
        }
        panic!("Expected typ {} found {}", typ, self.get_current_token().unwrap().typ);
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
                self.is(TokenType::SEMICOLON);
                Ok(Statement::Let {
                    name: name.value.unwrap(),
                    value: Box::from(val),
                })
            },
            TokenType::IDENT => {
                let name = self.get_current_token().unwrap();
                self.advance();
                self.is(TokenType::EQUALS);
                let val = self.expr();
                self.is(TokenType::SEMICOLON);
                Ok(Statement::Redef { name: name.value.unwrap(), value: Box::from(val) })
            },
            _ => {
                // Ok(Statement::Expr(())
                // Ok(self.expr())
                // println!("got {}", self.get_current_token().unwrap().typ);
                // Err(String::from("nothing wrong"))
            }
        }
    }

    fn expr(&mut self) -> Expr {
        let l = self.term(); // 12
        self.advance(); // -
        while matches!(
            self.get_current_token().unwrap().typ,
            TokenType::PLUS | TokenType::MINUS
        ) {
            let tok = self.get_current_token().unwrap().typ;
            self.advance();
            let r = self.term(); // 10
            self.advance();
            return match tok {
                TokenType::PLUS => Expr::BinOp {
                    left: Box::from(l),
                    op: Op::PLUS,
                    right: Box::from(r),
                },
                TokenType::MINUS => Expr::BinOp {
                    left: Box::from(l),
                    op: Op::MINUS,
                    right: Box::from(r),
                },
                _ => Expr::None {},
            };
        }
        l
    }

    fn term(&mut self) -> Expr {
        let l = self.factor();
        while matches!(
            self.get_current_token().unwrap().typ,
            TokenType::MULTIPLY | TokenType::DIVIDE
        ) {
            let tok = self.get_current_token().unwrap().typ;
            self.advance();
            let r = self.expr();
            return match tok {
                TokenType::MULTIPLY => Expr::BinOp {
                    left: Box::from(l),
                    op: Op::MULTIPLY,
                    right: Box::from(r),
                },
                TokenType::DIVIDE => Expr::BinOp {
                    left: Box::from(l),
                    op: Op::DIVIDE,
                    right: Box::from(r),
                },
                _ => Expr::None {},
            };
        }
        l
    }

    fn factor(&mut self) -> Expr {
        let z = self.get_current_token().unwrap();
        match z.typ {
            // TODO: handle other tokens
            TokenType::IDENT => Expr::Ident {
                name: (z.value.unwrap()),
            },
            TokenType::LITERAL => return Expr::Literal(Literal::String(z.value.unwrap())),
            TokenType::LPAREN => {
                self.advance();
                let e = self.expr();
                self.advance();
                e
            }
            _ => panic!("Parse error"),
        }
    }

    pub fn analyze(&mut self) {

    }

    pub fn build(&mut self) {
        while !matches!(self.get_current_token().unwrap().typ, TokenType::EOF) {
            let stmt = self.parse_stmt();
            match stmt {
                Ok(v) => match v {
                    // TODO: handle other tokens
                    Statement::Let { name, value } => {
                        println!("got var {}", name);
                    }
                    Statement::Redef { name, value } => {
                        println!("got redef var {}", name);
                    }
                    _ => {}
                },
                Err(e) => {
                    panic!("Error building ast {}", e);
                }
            };
        }
    }

    pub fn semantic_analyze(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::{Ast, Expr, Literal, Op, Statement};
    use crate::{
        parser::Parser,
        tokenizer::{Token, TokenType, Tokenizer},
    };

    fn tokenize_test_source() -> Vec<Token> {
        let mut t = Tokenizer::new("test_source.l");
        t.tokenize();
        t.tokens
    }

    fn expected_12_minus_6_expr() -> Expr {
        Expr::BinOp {
            left: Box::new(Expr::Literal(Literal::String(String::from("12")))),
            op: Op::MINUS,
            right: Box::new(Expr::Literal(Literal::String(String::from("6")))),
        }
    }

    fn assert_expr_eq(actual: &Expr, expected: &Expr) {
        match (actual, expected) {
            (Expr::Literal(Literal::String(a)), Expr::Literal(Literal::String(e))) => {
                assert_eq!(a, e);
            }
            (
                Expr::BinOp {
                    left: al,
                    op: ao,
                    right: ar,
                },
                Expr::BinOp {
                    left: el,
                    op: eo,
                    right: er,
                },
            ) => {
                assert_eq!(std::mem::discriminant(ao), std::mem::discriminant(eo));
                assert_expr_eq(al, el);
                assert_expr_eq(ar, er);
            }
            (Expr::Ident { name: a }, Expr::Ident { name: e }) => assert_eq!(a, e),
            _ => panic!("expression mismatch"),
        }
    }

    fn assert_let_stmt(stmt: &Statement, name: &str, expected_value: &Expr) {
        match stmt {
            Statement::Let { name: n, value } => {
                assert_eq!(n, name);
                assert_expr_eq(value, expected_value);
            }
            Statement::Redef { .. } => panic!("expected let statement, got redef"),
            Statement::Expr(_) => panic!("expected let statement, got expr"),
            Statement::None {} => panic!("expected let statement, got none"),
        }
    }

    fn parse_all_statements(tokens: Vec<Token>) -> Vec<Statement> {
        let mut ast = Ast::new(tokens);
        let mut statements = Vec::new();
        while !matches!(
            ast.get_current_token().unwrap().typ,
            TokenType::EOF
        ) {
            statements.push(ast.parse_stmt().unwrap());
        }
        statements
    }

    #[test]
    fn test_ast_build_from_test_source() {
        let tokens = tokenize_test_source();
        let _ast = Parser::new(tokens).parse();
    }

    #[test]
    fn test_parse_all_statements_from_test_source() {
        let statements = parse_all_statements(tokenize_test_source());
        assert_eq!(statements.len(), 2);
    }

    #[test]
    fn test_parse_first_let_statement() {
        let mut ast = Ast::new(tokenize_test_source());
        let stmt = ast.parse_stmt().unwrap();
        assert_let_stmt(&stmt, "abc", &expected_12_minus_6_expr());
    }

    #[test]
    fn test_parse_second_let_statement() {
        let mut ast = Ast::new(tokenize_test_source());
        ast.parse_stmt().unwrap();
        let stmt = ast.parse_stmt().unwrap();
        assert_let_stmt(&stmt, "def", &expected_12_minus_6_expr());
    }

    #[test]
    fn test_parse_expr_12_minus_6() {
        let tokens = tokenize_test_source();
        let expr_tokens: Vec<Token> = tokens[3..7].to_vec();
        let mut ast = Ast::new(expr_tokens);
        let expr = ast.expr();
        assert_expr_eq(&expr, &expected_12_minus_6_expr());
    }

    #[test]
    fn test_parse_reaches_eof_after_both_statements() {
        let mut ast = Ast::new(tokenize_test_source());
        ast.parse_stmt().unwrap();
        ast.parse_stmt().unwrap();
        assert!(matches!(
            ast.get_current_token().unwrap().typ,
            TokenType::EOF
        ));
    }
}
