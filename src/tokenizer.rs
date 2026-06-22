use core::fmt;
use std::fs;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LET,
    IDENT,
    EQUALS,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LITERAL,
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
            TokenType::LPAREN => write!(f, "("),
            TokenType::RPAREN => write!(f, ")"),
            TokenType::LITERAL => write!(f, "literal"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub value: Option<String>,
    pub line: i32,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Tokenizer {
    pub source: String,
    pub tokens: Vec<Token>,
    idx: usize,
    cur_line: i32,
}

impl Tokenizer {
    pub fn new(src: &str) -> Self {
        let contents = fs::read_to_string(src).expect("Should have been to read the file");

        Self {
            source: contents,
            tokens: Vec::new(),
            idx: 0,
            cur_line: 0,
        }
    }

    fn set_current_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    fn get_current_idx(&self) -> usize {
        self.idx
    }

    fn is_numeric(&self, s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    fn peek(&self, idx: usize) -> Option<char> {
        if self.source.len() > idx + 1 {
            Some(self.source.chars().nth((idx + 1) as usize)).unwrap()
        } else {
            None
        }
    }

    pub fn tokenize(self: &mut Self) {
        let mut cur = String::new();

        while self.get_current_idx() < self.source.len() {
            let lexeme = self.source.chars().nth(self.get_current_idx()).unwrap();

            cur.push(lexeme);

            match cur.as_str() {
                "=" => {
                    self.tokens.push(Token {
                        typ: TokenType::EQUALS,
                        value: Some(String::from("=")),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                }
                "+" => {
                    self.tokens.push(Token {
                        typ: TokenType::PLUS,
                        value: Some(String::from("+")),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                }
                "-" => {
                    self.tokens.push(Token {
                        typ: TokenType::MINUS,
                        value: Some(String::from("-")),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                }
                "*" => {
                    self.tokens.push(Token {
                        typ: TokenType::MULTIPLY,
                        value: Some(String::from("*")),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                }
                "/" => {
                    self.tokens.push(Token {
                        typ: TokenType::DIVIDE,
                        value: Some(String::from("/")),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                }
                ";" => {
                    self.tokens.push(Token {
                        typ: TokenType::SEMICOLON,
                        value: Some(String::from(";")),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                }
                "l" => {
                    if self.peek(self.get_current_idx()) == Some('e') {
                        if self.peek(self.get_current_idx() + 1) == Some('t') {
                            self.tokens.push(Token {
                                typ: TokenType::LET,
                                value: Some(String::from("let")),
                                line: self.cur_line,
                            });
                            cur = String::from("");
                            self.set_current_idx(self.get_current_idx() + 2);
                        }
                    }
                }
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => {
                    self.set_current_idx(self.get_current_idx() + 1);
                    while self.is_numeric(
                        &self
                            .source
                            .chars()
                            .nth(self.get_current_idx())
                            .unwrap()
                            .to_string(),
                    ) && self.get_current_idx() < self.source.len()
                    {
                        cur.push(self.source.chars().nth(self.get_current_idx()).unwrap());
                        self.set_current_idx(self.get_current_idx() + 1);
                    }
                    self.tokens.push(Token {
                        typ: TokenType::LITERAL,
                        value: Some(String::from(cur.clone())),
                        line: self.cur_line,
                    });
                    cur = String::from("");
                    continue;
                }
                "\n" => {
                    self.cur_line += 1;
                }
                " " | "\r" => {
                    cur = String::from("");
                }
                _ => {
                    if self.peek(self.get_current_idx()) == Some(' ')
                        || self.peek(self.get_current_idx()) == None
                        || self.peek(self.get_current_idx()) == Some(';')
                    {
                        self.tokens.push(Token {
                            typ: TokenType::IDENT,
                            value: Some(String::from(cur.clone())),
                            line: self.cur_line,
                        });
                        cur = String::from("");
                    }
                }
            };

            self.set_current_idx(self.get_current_idx() + 1);
        }
    }

    pub fn show_tokens(self: Self) {
        for z in self.tokens {
            println!("{} {}", z.typ.to_string(), z.value.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{Token, TokenType, Tokenizer};

    #[test]
    fn test_tokenizer_individual_tokens() {
        let mut t = Tokenizer::new("test_source.l");
        t.tokenize();

        let final_tokens: Vec<Token> = vec![
            Token {
                typ: TokenType::LET,
                value: Some(String::from("let")),
                line: 0,
            },
            Token {
                typ: TokenType::IDENT,
                value: Some(String::from("abc")),
                line: 0,
            },
            Token {
                typ: TokenType::EQUALS,
                value: Some(String::from("=")),
                line: 0,
            },
            Token {
                typ: TokenType::LITERAL,
                value: Some(String::from("12")),
                line: 0,
            },
            Token {
                typ: TokenType::MINUS,
                value: Some(String::from("-")),
                line: 0,
            },
            Token {
                typ: TokenType::LITERAL,
                value: Some(String::from("6")),
                line: 0,
            },
            Token {
                typ: TokenType::SEMICOLON,
                value: Some(String::from(";")),
                line: 0,
            },
        ];

        for (idx, token) in t.tokens.iter().enumerate() {
            assert_eq!(final_tokens.iter().nth(idx).unwrap().value, token.value);
            assert_eq!(
                final_tokens.iter().nth(idx).unwrap().typ.to_string(),
                token.typ.to_string()
            );
        }
    }

    #[test]
    fn test_tokenizer_result_token_count() {
        let mut t = Tokenizer::new("test_source.l");
        t.tokenize();
        assert_eq!(t.tokens.len(), 7);
    }
}
