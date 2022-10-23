mod tokens;
mod parser;
mod errors;

use crate::tokens::tokens::*;
use crate::errors::errors::*;
use crate::parser::ast::*;
use crate::parser::parser::*;


fn new_token(ttype: TokenType) -> Token {
    Token {typ: ttype, row: 0, col: 0}
}


fn factorial() -> Vec<Token> {
    vec![
        // fn factorial :: Int -> Int;
        new_token(TokenType::Fn),
        new_token(TokenType::Symbol(format!("factorial"))),
        new_token(TokenType::DoubleColon),
        new_token(TokenType::Symbol(format!("Int"))),
        new_token(TokenType::Arrow),
        new_token(TokenType::Symbol(format!("Int"))),
        new_token(TokenType::Semicolon),
        
        // fn factorial (0) => 1;
        new_token(TokenType::Fn),
        new_token(TokenType::Symbol(format!("factorial"))),
        new_token(TokenType::LParen),
        new_token(TokenType::Integer(0)),
        new_token(TokenType::RParen),
        new_token(TokenType::EqArrow),
        new_token(TokenType::Integer(1)),
        new_token(TokenType::Semicolon),
        
        // fn factorial (n) => n * factorial(n - 1);
        new_token(TokenType::Fn),
        new_token(TokenType::Symbol(format!("factorial"))),
        new_token(TokenType::LParen),
        new_token(TokenType::Symbol(format!("n"))),
        new_token(TokenType::RParen),
        new_token(TokenType::EqArrow),
        new_token(TokenType::Symbol(format!("n"))),
        new_token(TokenType::Star),
        new_token(TokenType::Symbol(format!("factorial"))),
        new_token(TokenType::LParen),
        new_token(TokenType::Symbol(format!("n"))),
        new_token(TokenType::Minus),
        new_token(TokenType::Integer(1)),
        new_token(TokenType::RParen),
        new_token(TokenType::Semicolon),
    ]
}

fn main() {
    match parse(factorial()) {
        Ok(tree) => print_tree(tree),
        Err(_) => println!("parser error")
    }
}
