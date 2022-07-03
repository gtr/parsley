mod tokens;
mod parser;

use crate::tokens::tokens::*;
use crate::parser::parser::*;
use crate::parser::ast::*;

/// 3;
#[allow(dead_code)]
fn test_one() -> Vec<Token> {
    vec![
        Token::Integer(3),
        Token::Semicolon,
    ]
}

/// 3 + 5;
#[allow(dead_code)]
fn test_two() -> Vec<Token> {
    vec![
        Token::Integer(3),
        Token::Plus,
        Token::Integer(5),
        Token::Semicolon,
    ]
}

/// 7 + 4 * 2;
#[allow(dead_code)]
fn test_three() -> Vec<Token> {
    vec![
        Token::Integer(7),
        Token::Plus,
        Token::Integer(4),
        Token::Star,
        Token::Integer(2),
        Token::Semicolon,
    ]
}

/// (7 + 4) * 2;
#[allow(dead_code)]
fn test_four() -> Vec<Token> {
    vec![
        Token::LeftParen,
        Token::Integer(7),
        Token::Plus,
        Token::Integer(4),
        Token::RightParen,
        Token::Star,
        Token::Integer(2),
        Token::Semicolon,
    ]
}

#[allow(dead_code)]

fn main() {
    let tokens = test_four();

    let mut p = Parser::new(tokens);
    let root = p.parse().unwrap();

    print_tree(&root);
}