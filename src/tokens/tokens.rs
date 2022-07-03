use std::fmt;

/// Token represents a token in the ivy programming language.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {

    // Atoms
    Integer(i32),   // 32
    String(String), // "hello"
    Symbol(String), // user-defined
    True,           // true
    False,          // false
    None,           // None

    // Operators
    Plus,           // +            
    PlusPlus,       // ++
    Minus,          // -
    MinusMimus,     // --
    Star,           // *
    Slash,          // /
    Bind,           // =
    Eq,             // ==
    Not,            // !
    NotEq,          // !=
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=
    And,            // &&
    Or,             // ||

    // Delimeters
    Pipe,           // |
    Arrow,          // ->
    Comma,          // ,
    Semicolon,      // ;
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftCurly,      // {
    RightCurly,     // }

    // Keywords
    Let,            // let
    Fn,             // fn
    If,             // if
    Data,           // data
    Enum,           // enum 
    Then,           // then
    Else,           // else
    Match,          // match
    Print,          // print
    PrintLn,        // println
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::Integer(i) => write!(f, "[Integer: {}]", i),
            Token::String(s) => write!(f, "[String: {}]", s),
            Token::Symbol(s) => write!(f, "[Symbol: {}]", s),
            Token::True => write!(f, "[True]"),
            Token::False => write!(f, "[False]"),
            Token::None => write!(f, "[None]"),
            
            Token::Plus => write!(f, "[+]"),
            Token::PlusPlus => write!(f, "[PlusPlus]"),
            Token::Minus => write!(f, "[-]"),
            Token::MinusMimus => write!(f, "[MinusMinus]"),
            Token::Star => write!(f, "[*]"),
            Token::Slash => write!(f, "[/]"),
            Token::Bind => write!(f, "[=]"),
            Token::Eq => write!(f, "[Eq]"),
            Token::Not => write!(f, "[Not]"),
            Token::NotEq => write!(f, "[!=]"),
            Token::Greater => write!(f, "[>]"),
            Token::GreaterEqual => write!(f, "[>=]"),
            Token::Less => write!(f, "[<]"),
            Token::LessEqual => write!(f, "[<=]"),
            Token::And => write!(f, "[&&]"),
            Token::Or => write!(f, "[||]"),
            
            Token::Pipe => write!(f, "[Pipe]"),
            Token::Arrow => write!(f, "[->]"),
            Token::Comma => write!(f, "[,]"),
            Token::Semicolon => write!(f, "[;]"),
            Token::LeftParen => write!(f, "[(]"),
            Token::RightParen => write!(f, "[)]"),
            Token::LeftBracket => write!(f, "[LeftBracket]"),
            Token::RightBracket => write!(f, "[RightBracket]"),
            Token::LeftCurly => write!(f, "[LeftCurly]"),
            Token::RightCurly => write!(f, "[RightCurly]"),
            
            Token::Let => write!(f, "[let]"),
            Token::Fn => write!(f, "[fn]"),
            Token::If => write!(f, "[If]"),
            Token::Then => write!(f, "[Then]"),
            Token::Else => write!(f, "[Else]"),
            Token::Data => write!(f, "[Data]"),
            Token::Enum => write!(f, "[Enum]"),
            Token::Match => write!(f, "[Match]"),
            Token::Print => write!(f, "[Print]"),
            Token::PrintLn => write!(f, "[Println]"),
        }
    }
}