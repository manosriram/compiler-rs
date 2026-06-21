use crate::{program::Program, tokenizer::Token};

pub struct Parser<'a> {
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens: tokens }
    }

    pub fn parse(&self) -> Program {
        Program::new()
    }
}
