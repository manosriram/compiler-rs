use crate::ast::Ast;
use crate::parser::Parser;
use crate::program::Program;
use crate::tokenizer::Tokenizer;

pub struct Tiny {
    pub tokenizer: Tokenizer,
}

// source -> tokens
// parser(tokens) -> AST -> build_symbol_table_from_ast
// validate(ast) -> generate_3AC

impl Tiny {
    pub fn new(src: &str) -> Self {
        let tokenizer = Tokenizer::new(src);
        Tiny {
            tokenizer: tokenizer,
        }
    }

    pub fn compile(&mut self) -> Ast {
        self.tokenizer.tokenize();
        Parser::new(self.tokenizer.tokens.clone()).parse()
    }
}
