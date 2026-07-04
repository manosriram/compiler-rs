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
        ast.analyze();
        ast.printsymtab();
        ast
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{
        ast::Ast,
        tokenizer::{Token, TokenType, Tokenizer},
    };

    fn tokenize_test_source() -> Vec<Token> {
        let mut t = Tokenizer::new("test_source.l");
        t.tokenize();
        t.tokens
    }

    #[test]
    fn test_parser_new_stores_tokens() {
        let tokens = tokenize_test_source();
        let parser = Parser::new(tokens.clone());
        assert_eq!(parser.tokens.len(), tokens.len());
        for (stored, original) in parser.tokens.iter().zip(tokens.iter()) {
            assert_eq!(
                std::mem::discriminant(&stored.typ),
                std::mem::discriminant(&original.typ)
            );
            assert_eq!(stored.value, original.value);
        }
    }

    #[test]
    fn test_parse_test_source_succeeds() {
        let parser = Parser::new(tokenize_test_source());
        let _ast: Ast = parser.parse();
    }

    #[test]
    fn test_parse_from_tokenizer_pipeline() {
        let mut tokenizer = Tokenizer::new("test_source.l");
        tokenizer.tokenize();
        assert_eq!(tokenizer.tokens.len(), 15);

        let parser = Parser::new(tokenizer.tokens);
        let _ast = parser.parse();
    }

    #[test]
    fn test_parse_can_be_called_multiple_times() {
        let parser = Parser::new(tokenize_test_source());
        let _ast1 = parser.parse();
        let _ast2 = parser.parse();
    }

    #[test]
    fn test_parse_leaves_parser_tokens_unchanged() {
        let tokens = tokenize_test_source();
        let parser = Parser::new(tokens.clone());
        let _ast = parser.parse();

        assert_eq!(parser.tokens.len(), tokens.len());
        assert!(matches!(parser.tokens.last().unwrap().typ, TokenType::EOF));
    }
}
