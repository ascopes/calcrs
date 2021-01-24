use std::fmt;

use crate::calcrs::token;

pub struct SyntaxError {
    pub what: char,
    pub pos: usize,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SyntaxError: found unexpected '{}' at {}", self.what, self.pos)
    }
}

#[derive(Debug)]
pub struct Lexer {
    text: String,
    pos: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer { text, pos: 0 }
    }

    pub fn next_token(&mut self) -> Result<token::Token, SyntaxError> {
        self.skip_ws();

        if self.pos >= self.text.len() {
            return Ok(token::Token { token_type: token::TokenType::Eof, text: "".to_string(), pos: self.pos });
        }

        let next_char = self.get_char();

        if next_char.is_ascii_digit() {
            return self.get_number();
        }

        if let Some(text) = self.text.get(self.pos..self.pos + 2) {
            let maybe_token_type = match text {
                "**" => Some(token::TokenType::Pow),
                "//" => Some(token::TokenType::IntDiv),
                _ => None
            };

            if let Some(token_type) = maybe_token_type {
                self.pos += 2;
                return Ok(token::Token { token_type, text: text.to_string(), pos: self.pos });
            }
        }

        let text = String::from(next_char);

        let result = match next_char {
            '+' => Ok(token::Token { token_type: token::TokenType::Plus, text, pos: self.pos }),
            '-' => Ok(token::Token { token_type: token::TokenType::Minus, text, pos: self.pos }),
            '*' => Ok(token::Token { token_type: token::TokenType::Mul, text, pos: self.pos }),
            '/' => Ok(token::Token { token_type: token::TokenType::Div, text, pos: self.pos }),
            '%' => Ok(token::Token { token_type: token::TokenType::Mod, text, pos: self.pos }),
            '(' => Ok(token::Token { token_type: token::TokenType::Lparen, text, pos: self.pos }),
            ')' => Ok(token::Token { token_type: token::TokenType::Rparen, text, pos: self.pos }),
            _ => return Err(SyntaxError { what: next_char, pos: self.pos })
        };

        self.pos += 1;
        return result;
    }

    fn skip_ws(&mut self) {
        let mut offset = 0;

        for c in self.text[self.pos..].chars() {
            if c.is_ascii_whitespace() {
                offset += 1;
            } else {
                break;
            }
        }

        self.pos += offset;
    }

    fn get_number(&mut self) -> Result<token::Token, SyntaxError> {
        let start = self.pos;
        let mut buff = String::new();
        let mut next = self.get_char();

        while next.is_ascii_digit() {
            buff.push(next);
            self.pos += 1;

            next = self.get_char();
        }

        // If the next char is a period, E, or e, then we have a float.
        // Otherwise we have an integer.
        if next != '.' || next != 'E' || next != 'e' {
            return Ok(token::Token { token_type: token::TokenType::Integer, text: buff, pos: start });
        }

        // Try eating a period.
        if next == '.' {
            buff.push('.');
            self.pos += 1;
            next = self.get_char();

            while next.is_ascii_digit() {
                buff.push(next);
                self.pos += 1;
                next = self.get_char();
            }
        }

        // Try eating an exponent.
        if next == 'e' || next == 'E' {
            buff.push(next);
            self.pos += 1;
            next = self.get_char();

            if next == '+' || next == '-' {
                buff.push(next);
                self.pos += 1;
                next = self.get_char();
            }

            if !next.is_ascii_digit() {
                return Err(SyntaxError { what: next, pos: self.pos });
            }

            while next.is_ascii_digit() {
                buff.push(next);
                self.pos += 1;
                next = self.get_char();
            }
        }

        return Ok(token::Token { token_type: token::TokenType::Float, text: buff, pos: start });
    }

    fn get_char(&self) -> char {
        self.text.chars().nth(self.pos).unwrap_or('\0')
    }
}