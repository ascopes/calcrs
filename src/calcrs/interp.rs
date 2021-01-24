use crate::calcrs::ast;
use crate::calcrs::token;

pub fn visit(node: &ast::Ast) -> f64 {
    match node {
        ast::Ast::Integer { token: _token, value } => value.clone() as f64,
        ast::Ast::Float { token: _token, value } => value.clone(),
        ast::Ast::BinOp { left, op, right} => visit_binop(&left, &op.token_type, &right),
        ast::Ast::UnaryOp { op, value} => visit_unaryop(&op.token_type, &value),
    }
}


fn visit_binop(left: &ast::Ast, op: &token::TokenType, right: &ast::Ast) -> f64 {
    match op {
        token::TokenType::Plus => visit(left) + visit(right),
        token::TokenType::Minus => visit(left) - visit(right),
        token::TokenType::Mul => visit(left) * visit(right),
        token::TokenType::Div => visit(left) / visit(right),
        token::TokenType::IntDiv => (visit(left) / visit(right)).floor(),
        token::TokenType::Pow => visit(left).powf(visit(right)),
        token::TokenType::Mod => visit(left) % visit(right),
        _ => unimplemented!()
    }
}

fn visit_unaryop(op: &token::TokenType, value: &ast::Ast) -> f64 {
    match op {
        token::TokenType::Plus => visit(value),
        token::TokenType::Minus => -visit(value),
        _ => unimplemented!()
    }
}