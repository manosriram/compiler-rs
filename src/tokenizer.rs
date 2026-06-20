use core::fmt;
use std::fs;

pub enum TokenType {
    LET,
    IDENT,
    EQUALS,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    SEMICOLON,
    LITERAL
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LET => write!(f, "let"),
            TokenType::IDENT => write!(f, "ident"),
            TokenType::EQUALS => write!(f, "="),
            TokenType::PLUS => write!(f, "+"),
            TokenType::MINUS => write!(f, "-"),
            TokenType::MULTIPLY => write!(f, "*"),
            TokenType::DIVIDE => write!(f, "/"),
            TokenType::SEMICOLON => write!(f, ";"),
            TokenType::LITERAL => write!(f, ""),
        }
    }
}

pub struct Token {
    pub typ: TokenType,
    pub value: Option<String>
}


pub struct Tokenizer {
    pub source: String,
    pub tokens: Vec<Token>
}

impl Tokenizer {

    pub fn new(src: &str) -> Self {
        let contents = fs::read_to_string(src).expect("Should have been to read the file");

        Self{
            source: contents,
            tokens: Vec::new(),
        }
    }

    pub fn lexer(self: &mut Self) {
        let mut cur = String::new();
        for lexeme in self.source.chars() {
            cur.push(lexeme);

            let len_before = self.tokens.len();
            match cur.as_str() {
                "let" => self.tokens.push(Token{
                    typ: TokenType::LET,
                    value: None
                }),
                "=" => self.tokens.push(Token{
                    typ: TokenType::EQUALS,
                    value: None
                }),
                "+" => self.tokens.push(Token{
                    typ: TokenType::PLUS,
                    value: None
                }),
                "-" => self.tokens.push(Token{
                    typ: TokenType::MINUS,
                    value: None
                }),
                "*" => self.tokens.push(Token{
                    typ: TokenType::MULTIPLY,
                    value: None
                }),
                "/" => self.tokens.push(Token{
                    typ: TokenType::DIVIDE,
                    value: None
                }),
                ";" => self.tokens.push(Token{
                    typ: TokenType::SEMICOLON,
                    value: None
                }),
                " " => {
                },
                _ => {

                }
            };

            let len_after = self.tokens.len();

            if len_before != len_after {
                cur = String::from("");
            }
        }

    }

    pub fn _show_tokens(self: Self) {
        println!("{}", self.tokens.len());

        for z in self.tokens {
            println!("{}", z.typ);
            // match z.value {
            // Some(x) => {println!("{}", x)},
            // None => {}
            // };

        }
    }
}

