mod tokens;

mod lexer;

use crate::{lexer::tokenize, tokens::Token};

fn main() {
    let program = include_str!("../examples/TEST.DIA");

    let tokens: Vec<Token> = tokenize(&program).collect();

    dbg!(tokens);
}
