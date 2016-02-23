use std::iter::{Peekable, Iterator};

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Decimal(String),
    LeftParen,
    RightParen,
    Assign,
    Operator(Op),
    Error {
        pos: i32,
        message: String,
    },
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Mult,
    Div,
}

pub struct Lexer<I: Iterator> {
    pos: i32,
    width: i32,
    input: Peekable<I>,
    operator_expected: bool,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(input: I) -> Lexer<I> {
        Lexer {
            input: input.peekable(),
            pos: 0,
            width: 0,
            operator_expected: false,
        }
    }

    fn lex_decimal(&mut self) -> Option<Token> {
        let mut token = String::new();
        if let Some(&'-') = self.input.peek() {
            self.width += 1;
            token.push('-');
            self.input.next();
        }
        loop {
            let symbol = match self.input.peek() {
                None => None,
                Some(ch) => {
                    if *ch >= '0' && *ch <= '9' {
                        Some(*ch)
                    } else {
                        None
                    }
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
        self.operator_expected = true;
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
                None => {
                    break;
                }
                Some(ch) => {
                    self.width += 1;
                    token.push(ch);
                    self.input.next();
                }
            }
        }
        self.pos += self.width;
        self.width = 0;
        self.operator_expected = true;
        Some(Token::Ident(token))
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut current_symbol;
        if let Some(s) = self.input.peek() {
            current_symbol = *s;
        } else {
            return None;
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
                self.operator_expected = false;
                Some(Token::LeftParen)
            }
            ')' => {
                self.input.next();
                self.pos += 1;
                self.operator_expected = true;
                Some(Token::RightParen)
            }
            '+' => {
                self.input.next();
                self.pos += 1;
                self.operator_expected = false;
                Some(Token::Operator(Op::Plus))
            }
            x if x == '-' && self.operator_expected => {
                self.input.next();
                self.pos += 1;
                self.operator_expected = false;
                Some(Token::Operator(Op::Minus))
            }
            '*' => {
                self.input.next();
                self.pos += 1;
                self.operator_expected = false;
                Some(Token::Operator(Op::Mult))
            }
            '/' => {
                self.input.next();
                self.pos += 1;
                self.operator_expected = false;
                Some(Token::Operator(Op::Div))
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
