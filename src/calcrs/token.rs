#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    Eof,

    Integer,
    Float,

    Plus,
    Minus,
    Mul,
    Div,
    IntDiv,
    Pow,
    Mod,

    Lparen,
    Rparen,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub pos: usize,
}
