mod tokens;
mod parser;

use crate::tokens::tokens::*;
use crate::parser::parser::*;
use crate::parser::ast::*;

/// 3;
#[allow(dead_code)]
fn test_1() -> Vec<Token> {
    vec![
        Token::Integer(3),
        Token::Semicolon,
    ]
}

/// (7 + 4) * 2;
#[allow(dead_code)]
fn test_2() -> Vec<Token> {
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

/// let variable = (7 + 4) * 2; 3 + 5;
#[allow(dead_code)]
fn test_3() -> Vec<Token> {
    vec![
        Token::Let,
        Token::Symbol("variable".to_string()),
        Token::Bind,
        Token::LeftParen,
        Token::Integer(7),
        Token::Plus,
        Token::Integer(4),
        Token::RightParen,
        Token::Star,
        Token::Integer(2),
        Token::Semicolon,
        Token::Integer(3),
        Token::Plus,
        Token::Integer(5),
        Token::Semicolon,
    ]
}

/// let func = fn() -> (7 + 4) * 2;
#[allow(dead_code)]
fn test_4() -> Vec<Token> {
    vec![
        Token::Let,
        Token::Symbol("func".to_string()),
        Token::Bind,
        Token::Fn,
        Token::LeftParen,
        Token::RightParen,
        Token::Arrow,
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

// let square = fn(x) -> x * x;
#[allow(dead_code)]
fn test_5() -> Vec<Token> {
    vec![
        Token::Let,
        Token::Symbol("square".to_string()),
        Token::Bind,
        Token::Fn,
        Token::LeftParen,
        Token::Symbol("x".to_string()),
        Token::RightParen,
        Token::Arrow,
        Token::Symbol("x".to_string()),
        Token::Star,
        Token::Symbol("x".to_string()),
        Token::Semicolon,
    ]
}

// let add = fn(a, b) -> a + b;
#[allow(dead_code)]
fn test_6() -> Vec<Token> {
    vec![
        Token::Let,        
        Token::Symbol("add".to_string()),
        Token::Bind,
        Token::Fn,
        Token::LeftParen,
        Token::Symbol("a".to_string()),
        Token::Comma,
        Token::Symbol("b".to_string()),
        Token::RightParen,
        Token::Arrow,    
        Token::Symbol("a".to_string()),
        Token::Plus,
        Token::Symbol("b".to_string()),
        Token::Semicolon,
    ]
}

// let (a, b) = fn(a, b) -> a + b;
#[allow(dead_code)]
fn test_7() -> Vec<Token> {
    vec![
        Token::Let,        
        Token::LeftParen,
        Token::Symbol("a".to_string()),
        Token::Comma,
        Token::Symbol("b".to_string()),
        Token::RightParen,
        Token::Bind,
        Token::Fn,
        Token::LeftParen,
        Token::Symbol("a".to_string()),
        Token::Comma,
        Token::Symbol("b".to_string()),
        Token::RightParen,
        Token::Arrow,    
        Token::Symbol("a".to_string()),
        Token::Plus,
        Token::Symbol("b".to_string()),
        Token::Semicolon,
    ]
}

// (3 + 3) != (2 * 3);
#[allow(dead_code)]
fn test_8() -> Vec<Token> {
    vec![
        Token::LeftParen,
        Token::Integer(3),
        Token::Plus,
        Token::Integer(3),
        Token::RightParen,
        Token::NotEq,
        Token::LeftParen,
        Token::Integer(2),
        Token::Star,
        Token::Integer(3),
        Token::RightParen,
        Token::Semicolon,
    ]
}

// if (true && false) then true else false;
#[allow(dead_code)]
fn test_9() -> Vec<Token> {
    vec![
        Token::If,
        Token::LeftParen,
        Token::True,
        Token::And,
        Token::False,
        Token::RightParen,
        Token::Then,
        Token::True,
        Token::Else,
        Token::False,
        Token::Semicolon,
    ]
}

// let area = square(3);
#[allow(dead_code)]
fn test_10() -> Vec<Token> {
    vec![
        Token::Let,
        Token::Symbol("area".to_string()),
        Token::Bind,
        Token::Symbol("square".to_string()),
        Token::LeftParen,
        Token::Integer(3),
        Token::RightParen,
        Token::Semicolon,
    ]
}

// if (true) then !(true and false or true);
#[allow(dead_code)]
fn test_11() -> Vec<Token> {
    vec![
        Token::If,
        Token::LeftParen,
        Token::True,
        Token::RightParen,
        Token::Then,
        Token::Not,
        Token::LeftParen,
        Token::True,
        Token::And,
        Token::False,
        Token::Or,
        Token::True,
        Token::RightParen,
        Token::Semicolon,
    ]
}

fn main() {
    let tokens = test_6();

    let mut p = Parser::new(tokens);
    let root = p.parse().unwrap();

    print_tree(&root);
}