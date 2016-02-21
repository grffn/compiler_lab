use std::iter::{Peekable, Iterator};

pub enum Token {
    Ident(String),
    Decimal(String),
    LeftParen,
    RightParen,
    Assign,
    Operator(String),
    Error {
        pos: i32,
        message: String,
    },
}

pub struct Lexer<I: Iterator> {
    pos: i32,
    width: i32,
    input: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn new(input: I) -> Lexer<I> {
        Lexer {
            input: input.peekable(),
            pos: 0,
            width: 0,
        }
    }

    fn lex_decimal(&mut self) -> Option<Token> {
        let mut token = String::new();
        if let Some(&'-') = self.input.peek() {
            self.width += 1;
            token.push('-');
            self.input.next();
        }
        loop  {
            let symbol = match self.input.peek() {
                None => None,
                Some(ch) => {
                    if *ch >= '0' && *ch <= '9' {Some(*ch)}
                    else {None}
                }
            };
            match symbol {
                None => break,
                Some(ch) => {
                    self.width += 1;
                    token.push(ch);
                    self.input.next();
                }
            }
        }
        self.pos += self.width;
        self.width = 0;
        Some(Token::Decimal(token))
    }

    fn lex_ident(&mut self) -> Option<Token> {
        let mut token = String::new();
        loop {
            let symbol = match self.input.peek() {
                None => None,
                Some(ch) => {
                    if ch.is_alphabetic() || (*ch >= '0' && *ch <= '9') || *ch == '_' {
                        Some(*ch)
                    } else {
                        None
                    }
                }
            };
            match symbol {
                None => {break;}
                Some(ch) => {
                    self.width += 1;
                    token.push(ch);
                    self.input.next();
                }
            }
        }
        self.pos += self.width;
        self.width = 0;
        Some(Token::Ident(token))
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut current_symbol = '\0';
        if let Some(s) = self.input.peek() {
            current_symbol = *s;
        }
        while current_symbol.is_whitespace() {
            self.pos += 1;
            self.input.next();
            if let Some(s) = self.input.peek() {
                current_symbol = *s;
            }
        }
        match current_symbol {
            '(' => {
                self.input.next();
                self.pos += 1;
                Some(Token::LeftParen)
            }
            ')' => {
                self.input.next();
                self.pos += 1;
                Some(Token::RightParen)
            }
            '-' | '0'...'9' => self.lex_decimal(),
            ':' => {
                self.input.next();
                if let Some(&'=') = self.input.peek() {
                    self.input.next();
                    self.pos += 2;
                    Some(Token::Assign)
                } else {
                    Some(Token::Error {
                        pos: self.pos + 1,
                        message: "Unknown symbol".to_string(),
                    })
                }
            }
            x if x.is_alphabetic() || x == '_' => self.lex_ident(),
            _ => {
                Some(Token::Error {
                    pos: self.pos,
                    message: "Unknown token".to_string(),
                })
            }

        }
    }
}
