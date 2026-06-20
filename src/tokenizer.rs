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
            TokenType::LITERAL => write!(f, ""),
        }
    }
}

pub struct Token {
    pub typ: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    pub source: String,
    idx: usize,
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(src: &str) -> Self {
        let contents = fs::read_to_string(src).expect("Should have been to read the file");

        Self {
            source: contents,
            tokens: Vec::new(),
            idx: 0,
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

    pub fn lexer(self: &mut Self) {
        let mut cur = String::new();

        while self.get_current_idx() < self.source.len() {
            let lexeme = self.source.chars().nth(self.get_current_idx()).unwrap();

            cur.push(lexeme);

            match cur.as_str() {
                "=" => {
                    self.tokens.push(Token {
                        typ: TokenType::EQUALS,
                        value: Some(String::from("=")),
                    });
                    cur = String::from("");
                }
                "+" => {
                    self.tokens.push(Token {
                        typ: TokenType::PLUS,
                        value: Some(String::from("+")),
                    });
                    cur = String::from("");
                }
                "-" => {
                    self.tokens.push(Token {
                        typ: TokenType::MINUS,
                        value: Some(String::from("-")),
                    });
                    cur = String::from("");
                }
                "*" => {
                    self.tokens.push(Token {
                        typ: TokenType::MULTIPLY,
                        value: Some(String::from("*")),
                    });
                    cur = String::from("");
                }
                "/" => {
                    self.tokens.push(Token {
                        typ: TokenType::DIVIDE,
                        value: Some(String::from("/")),
                    });
                    cur = String::from("");
                }
                ";" => {
                    self.tokens.push(Token {
                        typ: TokenType::SEMICOLON,
                        value: Some(String::from(";")),
                    });
                    cur = String::from("");
                }
                "l" => {
                    if self.peek(self.get_current_idx()) == Some('e') {
                        if self.peek(self.get_current_idx() + 1) == Some('t') {
                            self.tokens.push(Token {
                                typ: TokenType::LET,
                                value: Some(String::from("let")),
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
                    });
                    cur = String::from("");
                    continue;
                }
                " " => {
                    cur = String::from("");
                }
                _ => {
                    if self.peek(self.get_current_idx()) == Some(' ') {
                        self.tokens.push(Token {
                            typ: TokenType::IDENT,
                            value: Some(String::from(cur.clone())),
                        });
                        cur = String::from("");
                    }
                }
            };

            self.set_current_idx(self.get_current_idx() + 1);
        }
    }

    pub fn _show_tokens(self: Self) {
        for z in self.tokens {
            println!("{} {}", z.typ.to_string(), z.value.unwrap());
        }
    }
}
