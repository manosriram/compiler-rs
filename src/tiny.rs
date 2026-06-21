use crate::parser::Parser;
use crate::program::Program;
use crate::tokenizer::Tokenizer;

pub struct Tiny {
    pub tokenizer: Tokenizer,
}

impl Tiny {
    pub fn new(src: &str) -> Self {
        let tokenizer = Tokenizer::new(src);
        Tiny {
            tokenizer: tokenizer,
        }
    }

    pub fn compile(&mut self) -> Program {
        self.tokenizer.tokenize();
        Parser::new(&self.tokenizer.tokens).parse()
    }
}
