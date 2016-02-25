use std::iter::{Peekable, Iterator};

#[derive(Debug)]
pub enum Token {
    Ident(String),
    Decimal(String),
    Exp(String),
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
    buffer: String,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(input: I) -> Lexer<I> {
        Lexer {
            input: input.peekable(),
            pos: 0,
            width: 0,
            operator_expected: false,
            buffer: String::new(),
        }
    }

    fn accept<F>(&mut self, f: F) -> bool
        where F: Fn(char) -> bool
    {
        match self.input.peek() {
            Some(&ch) => {
                if !f(ch) {
                    return false;
                }
                self.buffer.push(ch);
                self.width += 1;
                self.input.next();
                true
            }
            None => false,
        }

    }

    fn accept_run<F>(&mut self, f: F)
        where F: Fn(char) -> bool
    {
        while let Some(&ch) = self.input.peek() {
            if !f(ch) {
                break;
            }
            self.buffer.push(ch);
            self.width += 1;
            self.input.next();
        }
    }

    fn lex_decimal(&mut self) -> Option<Token> {
        self.accept(|ch| ch == '-');
        let is_digit = |ch| ch >= '0' && ch <= '9';
        self.accept_run(&is_digit);
        if self.accept(|ch| ch == '.') {
            self.accept_run(&is_digit);
        }
        let mut is_exp = false;
        if self.accept(|ch| ch == 'e' || ch == 'E'){
            self.accept_run(&is_digit);
            is_exp = true;
        }
        self.pos += self.width;
        self.width = 0;
        self.operator_expected = true;
        if is_exp {
            Some(Token::Exp(self.buffer.clone()))
        } else {
            Some(Token::Decimal(self.buffer.clone()))
        }

    }

    fn lex_ident(&mut self) -> Option<Token> {
        self.accept_run(|ch| {
            ch.is_alphabetic() || (ch >= '0' && ch <= '9') || ch == '_'
        });
        self.pos += self.width;
        self.width = 0;
        self.operator_expected = true;
        Some(Token::Ident(self.buffer.clone()))
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut current_symbol;
        self.buffer = String::new();
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
