use crate::calcrs::ast;
use crate::calcrs::ast::Ast::UnaryOp;
use crate::calcrs::lex;
use crate::calcrs::lex::SyntaxError;
use crate::calcrs::token;
use crate::calcrs::token::TokenType;

pub struct ParserError {
    pub why: String,
    pub pos: usize,
}

pub struct Parser {
    lexer: lex::Lexer,
    current_token: token::Token,
}

impl Parser {
    pub fn new(mut lexer: lex::Lexer) -> Result<Self, ParserError> {
        return match lexer.next_token() {
            Ok(current_token) => Ok(Parser { lexer, current_token }),
            Err(syntax_err) => Err(Parser::syntax_to_parser_err(syntax_err))
        };
    }

    pub fn parse(&mut self) -> Result<ast::Ast, ParserError> {
        return self.expr();
    }

    fn expr(&mut self) -> Result<ast::Ast, ParserError> {
        let mut term = self.term();

        if term.is_err() {
            return Err(term.err().unwrap());
        }

        while [token::TokenType::Plus, token::TokenType::Minus].contains(&self.current_token.token_type) {
            let op = self.eat(self.current_token.token_type.clone())?;

            let right = self.term();

            if right.is_err() {
                return Err(right.err().unwrap());
            }

            term = Ok(ast::Ast::BinOp {
                left: Box::new(term?),
                op,
                right: Box::new(right?),
            });
        }

        return term;
    }

    fn term(&mut self) -> Result<ast::Ast, ParserError> {
        let mut unary = self.unary();

        if unary.is_err() {
            return Err(unary.err().unwrap());
        }

        while [
            token::TokenType::Mul,
            token::TokenType::Div,
            token::TokenType::Pow,
            token::TokenType::IntDiv,
            token::TokenType::Mod,
        ].contains(&self.current_token.token_type) {
            let op = self.eat(self.current_token.token_type.clone())?;

            let right = self.unary();

            if right.is_err() {
                return Err(right.err().unwrap());
            }

            unary = Ok(ast::Ast::BinOp {
                left: Box::new(unary?),
                op,
                right: Box::new(right?),
            });
        }

        return unary;
    }

    fn unary(&mut self) -> Result<ast::Ast, ParserError> {
        if [
            token::TokenType::Plus,
            token::TokenType::Minus
        ].contains(&self.current_token.token_type) {
            let op = self.eat(self.current_token.token_type.clone())?;
            let expr = self.unary();

            if expr.is_err() {
                return Err(expr.err().unwrap());
            }

            return Ok(UnaryOp { op, value: Box::new(expr?) });
        }

        if self.current_token.token_type == token::TokenType::Lparen {
            self.eat(token::TokenType::Lparen)?;
            let expr = self.expr();

            if let Err(err) = self.eat(token::TokenType::Rparen) {
                return Err(err);
            }

            return expr;
        }

        if [
            token::TokenType::Integer,
            token::TokenType::Float
        ].contains(&self.current_token.token_type) {
            return self.number();
        }

        return self.unexpected_token()
    }

    fn number(&mut self) -> Result<ast::Ast, ParserError> {
        return match self.current_token.token_type {
            token::TokenType::Integer => {
                let token = self.eat(TokenType::Integer)?;
                let value: i64 = token.text.parse().unwrap();
                Ok(ast::Ast::Integer {token, value})
            }

            token::TokenType::Float => {
                let token = self.eat(TokenType::Float)?;
                let value: f64 = token.text.parse().unwrap();
                Ok(ast::Ast::Float {token, value})
            }

            _ => {
                let err = format!("expected numeric token type but received {:?}", self.current_token);
                Err(ParserError { why: err, pos: self.current_token.pos })
            }
        }
    }

    fn unexpected_token(&mut self) -> Result<ast::Ast, ParserError> {
        let msg = format!("Unexpected token type {:?}", self.current_token.token_type);
        return Err(ParserError { why: msg, pos: self.current_token.pos })
    }

    fn eat(&mut self, expect: token::TokenType) -> Result<token::Token, ParserError> {
        if self.current_token.token_type == expect {
            return match self.lexer.next_token() {
                Ok(mut token) => {
                    std::mem::swap(&mut self.current_token, &mut token);
                    Ok(token)
                }
                Err(syntax_err) => Err(Parser::syntax_to_parser_err(syntax_err))
            };
        }

        let err = format!("expected token type {:?} but received {:?}", expect, self.current_token);

        return Err(ParserError { why: err, pos: self.current_token.pos });
    }

    fn syntax_to_parser_err(syntax_err: SyntaxError) -> ParserError {
        let why = format!(
            "syntax error input - unexpected character '{}' at {}",
            syntax_err.what,
            syntax_err.pos
        );

        ParserError { why, pos: syntax_err.pos }
    }
}
