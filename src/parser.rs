use crate::{ast::Ast, program::Program, tokenizer::Token};

pub struct Parser {
    tokens: Vec<Token>,

}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens }
    }


    pub fn parse(&self) -> Ast {
        let mut ast = Ast::new(self.tokens.clone());
        ast.build();
        ast
    }
}
