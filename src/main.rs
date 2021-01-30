use std::io::{self, BufRead};

use calcrs::lex;
use calcrs::parser;
use calcrs::interp;
use crate::calcrs::parser::ParserError;

mod calcrs;

fn parser_error(parser_error: ParserError) {
    println!("Parser Error: {} in input at {}", parser_error.why, parser_error.pos);
    sys::process::exit(1);
}

fn main() {
    for maybe_line in io::stdin().lock().lines() {
        if let Ok(line) = maybe_line {

            if line.starts_with("/quit") || line.starts_with("/exit") {
                break;
            }

            let lexer = lex::Lexer::new(line);
            let maybe_parser = parser::Parser::new(lexer);

            if let Err(err) = maybe_parser {
                parser_error(err);
            } else if let Ok(mut parser) = maybe_parser {
                let maybe_tree = parser.parse();

                if let Err(err) = maybe_tree {
                    parser_error(err);
                } else if let Ok(tree) = maybe_tree {
                    let result = interp::visit(&tree);
                    println!("{}", result);
                }
            }
        } else {
            break;
        }
    }
}
