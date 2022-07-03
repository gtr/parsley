use std::fmt;

/// Token represents a token in the ivy programming language.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {

    // Atoms
    Integer(i32),   // 32
    String(String), // "hello"
    // Symbol(String), // user-defined
    // True,           // true
    // False,          // false
    // None,           // None

    // Operators
    Plus,           // +            
    // PlusPlus,       // ++
    Minus,          // -
    // MinusMimus,     // --
    Star,           // *
    Slash,          // /
    // Bind,           // =
    // Eq,             // ==
    // Not,            // !
    // NotEq,          // !=
    // Greater,        // >
    // GreaterEqual,   // >=
    // Less,           // <
    // LessEqual,      // <=

    // Delimeters
    // Pipe,           // |
    // Arrow,          // ->
    // Comma,          // ,
    Semicolon,      // ;
    LeftParen,      // (
    RightParen,     // )
    // LeftBracket,    // [
    // RightBracket,   // ]
    // LeftCurly,      // {
    // RightCurly,     // }

    // Keywords
    // Let,            // let
    // Fn,             // fn
    // If,             // if
    // Data,           // data
    // Enum,           // enum 
    // Then,           // then
    // Else,           // else
    // Match,          // match
    // Print,          // print
    // PrintLn,        // println
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::Integer(i) => write!(f, "[Integer: {}]", i),
            Token::String(s) => write!(f, "[String: {}]", s),
            // Token::Symbol(s) => write!(f, "[Symbol: {}]", s),
            // Token::True => write!(f, "[True]"),
            // Token::False => write!(f, "[False]"),
            // Token::None => write!(f, "[None]"),
            
            Token::Plus => write!(f, "[Plus]"),
            // Token::PlusPlus => write!(f, "[PlusPlus]"),
            Token::Minus => write!(f, "[Minus]"),
            // Token::MinusMimus => write!(f, "[MinusMinus]"),
            Token::Star => write!(f, "[Star]"),
            Token::Slash => write!(f, "[Slash]"),
            // Token::Bind => write!(f, "[Bind]"),
            // Token::Eq => write!(f, "[Eq]"),
            // Token::Not => write!(f, "[Not]"),
            // Token::NotEq => write!(f, "[NotEq]"),
            // Token::Greater => write!(f, "[Greater]"),
            // Token::GreaterEqual => write!(f, "[GreaterEqual]"),
            // Token::Less => write!(f, "[Less]"),
            // Token::LessEqual => write!(f, "[LessEqual]"),
            
            // Token::Pipe => write!(f, "[Pipe]"),
            // Token::Arrow => write!(f, "[Arrow]"),
            // Token::Comma => write!(f, "[Comma]"),
            Token::Semicolon => write!(f, "[Semicolon]"),
            Token::LeftParen => write!(f, "[LeftParen]"),
            Token::RightParen => write!(f, "[RightParen]"),
            // Token::LeftBracket => write!(f, "[LeftBracket]"),
            // Token::RightBracket => write!(f, "[RightBracket]"),
            // Token::LeftCurly => write!(f, "[LeftCurly]"),
            // Token::RightCurly => write!(f, "[RightCurly]"),
            
            // Token::Let => write!(f, "[Let]"),
            // Token::Fn => write!(f, "[Fn]"),
            // Token::If => write!(f, "[If]"),
            // Token::Then => write!(f, "[Then]"),
            // Token::Else => write!(f, "[Else]"),
            // Token::Data => write!(f, "[Data]"),
            // Token::Enum => write!(f, "[Enum]"),
            // Token::Match => write!(f, "[Match]"),
            // Token::Print => write!(f, "[Print]"),
            // Token::PrintLn => write!(f, "[Println]"),
        }
    }
}