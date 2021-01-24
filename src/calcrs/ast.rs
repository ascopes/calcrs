use crate::calcrs::token;

#[derive(Debug)]
pub enum Ast {
    Integer { token: token::Token, value: i64 },
    Float { token: token::Token, value: f64 },

    UnaryOp { op: token::Token, value: Box<Ast> },
    BinOp { left: Box<Ast>, op: token::Token, right: Box<Ast> },
}
