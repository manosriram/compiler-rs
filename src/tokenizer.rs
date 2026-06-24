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
    EOF,
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
            TokenType::EOF => write!(f, "eof"),
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
                    cur = String::from("");
                }
                " " | "\r" | "\t" => {
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

        self.tokens.push(Token {
            typ: TokenType::EOF,
            value: Some(String::from("")),
            line: self.cur_line,
        });
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

    fn tokenize(source: &str) -> Tokenizer {
        let mut t = Tokenizer {
            source: String::from(source),
            tokens: Vec::new(),
            idx: 0,
            cur_line: 0,
        };
        t.tokenize();
        t
    }

    fn assert_token_types(tokens: &[Token], expected: &[TokenType]) {
        assert_eq!(tokens.len(), expected.len());
        for (token, expected_type) in tokens.iter().zip(expected.iter()) {
            assert_eq!(
                std::mem::discriminant(&token.typ),
                std::mem::discriminant(expected_type)
            );
        }
    }

    fn assert_token_values(tokens: &[Token], expected: &[&str]) {
        assert_eq!(tokens.len(), expected.len());
        for (token, expected_value) in tokens.iter().zip(expected.iter()) {
            assert_eq!(token.value.as_deref(), Some(*expected_value));
        }
    }

    fn assert_same_token_types(left: &[Token], right: &[Token]) {
        assert_eq!(left.len(), right.len());
        for (l, r) in left.iter().zip(right.iter()) {
            assert_eq!(
                std::mem::discriminant(&l.typ),
                std::mem::discriminant(&r.typ)
            );
        }
    }

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
            Token {
                typ: TokenType::LET,
                value: Some(String::from("let")),
                line: 0,
            },
            Token {
                typ: TokenType::IDENT,
                value: Some(String::from("def")),
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
            Token {
                typ: TokenType::EOF,
                value: Some(String::from("")),
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
        assert_eq!(t.tokens.len(), 15);
    }

    #[test]
    fn test_tokenizer_loads_test_source_file() {
        let t = Tokenizer::new("test_source.l");
        assert!(t.source.contains("let abc = 12 - 6;"));
        assert!(t.source.contains("let def = 12 - 6;"));
    }

    #[test]
    fn test_tokenize_empty_source_produces_eof_only() {
        let t = tokenize("");
        assert_eq!(t.tokens.len(), 1);
        assert_token_types(&t.tokens, &[TokenType::EOF]);
        assert_eq!(t.tokens[0].value.as_deref(), Some(""));
    }

    #[test]
    fn test_tokenize_ends_with_eof() {
        let t = tokenize("let x = 1;");
        let last = t.tokens.last().unwrap();
        assert_eq!(
            std::mem::discriminant(&last.typ),
            std::mem::discriminant(&TokenType::EOF)
        );
    }

    #[test]
    fn test_tokenize_arithmetic_operators() {
        let t = tokenize("1 + 2 - 3 * 4 / 5 ");
        assert_token_types(
            &t.tokens,
            &[
                TokenType::LITERAL,
                TokenType::PLUS,
                TokenType::LITERAL,
                TokenType::MINUS,
                TokenType::LITERAL,
                TokenType::MULTIPLY,
                TokenType::LITERAL,
                TokenType::DIVIDE,
                TokenType::LITERAL,
                TokenType::EOF,
            ],
        );
        assert_token_values(
            &t.tokens,
            &["1", "+", "2", "-", "3", "*", "4", "/", "5", ""],
        );
    }

    #[test]
    fn test_tokenize_multidigit_literals() {
        let t = tokenize("100 2500 ");
        assert_token_types(
            &t.tokens,
            &[TokenType::LITERAL, TokenType::LITERAL, TokenType::EOF],
        );
        assert_token_values(&t.tokens, &["100", "2500", ""]);
    }

    #[test]
    fn test_tokenize_whitespace_is_ignored() {
        let compact = tokenize("let x = 1;");
        let spaced = tokenize("  let   x   =   1   ;  ");
        assert_same_token_types(&compact.tokens, &spaced.tokens);
        assert_token_values(
            &compact.tokens,
            &["let", "x", "=", "1", ";", ""],
        );
    }

    #[test]
    fn test_tokenize_simple_let_statement() {
        let t = tokenize("let x = 1;");
        assert_token_types(
            &t.tokens,
            &[
                TokenType::LET,
                TokenType::IDENT,
                TokenType::EQUALS,
                TokenType::LITERAL,
                TokenType::SEMICOLON,
                TokenType::EOF,
            ],
        );
        assert_token_values(&t.tokens, &["let", "x", "=", "1", ";", ""]);
    }

    #[test]
    fn test_tokenize_let_expression_with_subtraction() {
        let t = tokenize("let abc = 12 - 6;");
        assert_token_types(
            &t.tokens,
            &[
                TokenType::LET,
                TokenType::IDENT,
                TokenType::EQUALS,
                TokenType::LITERAL,
                TokenType::MINUS,
                TokenType::LITERAL,
                TokenType::SEMICOLON,
                TokenType::EOF,
            ],
        );
        assert_token_values(&t.tokens, &["let", "abc", "=", "12", "-", "6", ";", ""]);
    }

    #[test]
    fn test_tokenize_tracks_line_numbers() {
        let t = tokenize("let a = 1;\nlet b = 2;");
        assert_eq!(t.tokens[0].line, 0);
        assert_eq!(t.tokens[4].line, 0);
        assert_eq!(t.tokens[5].line, 1);
        assert_eq!(t.tokens[t.tokens.len() - 1].line, 1);
    }

    #[test]
    fn test_tokenize_multiple_statements_match_test_source_shape() {
        let t = tokenize("let abc = 12 - 6;\nlet def = 12 - 6;");
        assert_eq!(t.tokens.len(), 15);
        assert_token_types(
            &t.tokens,
            &[
                TokenType::LET,
                TokenType::IDENT,
                TokenType::EQUALS,
                TokenType::LITERAL,
                TokenType::MINUS,
                TokenType::LITERAL,
                TokenType::SEMICOLON,
                TokenType::LET,
                TokenType::IDENT,
                TokenType::EQUALS,
                TokenType::LITERAL,
                TokenType::MINUS,
                TokenType::LITERAL,
                TokenType::SEMICOLON,
                TokenType::EOF,
            ],
        );
    }
}
