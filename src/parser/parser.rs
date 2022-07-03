use crate::tokens::tokens::*;
use crate::parser::ast::*;
// use crate::parser::ast::*;
// use crate::parser::ast::Expression::*;

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Parser {
        println!("{:?}", tokens);
        Parser{
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let mut root = Node::new(GrammarItem::Root);

        while !self.is_done() {
            let expr = self.parse_expression()?;
            println!("parse : pushing {:?}", expr);
            root.children.push(expr);
        }

        Ok(root)
    }

    /// Parses an expression based on the following rule:
    /// 
    /// <expression> ::= <addition> ';' ;
    /// 
    fn parse_expression(&mut self) -> Result<Node, String> {
        let add = self.parse_addition();

        self.expect_semicolon()?;

        return add;

    }
    
    /// Parses an addition expression based on the following rule:
    /// 
    /// <addition> ::= <mult> [ ( '+' | '-' ) <mult> ]* ;
    /// 
    fn parse_addition(&mut self) -> Result<Node, String> {
        let lhs = self.parse_mult()?;
        match self.peek() {
            Some(Token::Plus) => {
                let mut n = Node::new(GrammarItem::Plus);
                self.next();
                let rhs = self.parse_mult()?;
                println!("parse_addition (+) : pushing {:?}", lhs);
                n.children.push(lhs);
                println!("parse_addition (+) : pushing {:?}", rhs);
                n.children.push(rhs);
                Ok(n)
            },
            Some(Token::Minus) => {
                let mut n = Node::new(GrammarItem::Minus);
                self.next();
                let rhs = self.parse_mult()?;
                println!("parse_addition (-) : pushing {:?}", lhs);
                n.children.push(lhs);
                println!("parse_addition (-) : pushing {:?}", rhs);
                n.children.push(rhs);
                Ok(n)
            },
            // Some(_) => Err(format!("expected + or -")),
            _ => Ok(lhs),
        }
    }

    /// Parses a multiplication expression based on the following rule:
    /// 
    /// <mult> ::= <factor> [ ( '*' | '/' ) <factor> ]* ;
    /// 
    fn parse_mult(&mut self) -> Result<Node, String> {
        let lhs = self.parse_factor()?;
        match self.peek() {
            Some(Token::Star) => {
                let mut n = Node::new(GrammarItem::Multiply);
                self.next();
                let rhs = self.parse_factor()?;
                println!("parse_mult (*) : pushing {:?}", lhs);
                n.children.push(lhs);
                println!("parse_mult (*) : pushing {:?}", rhs);
                n.children.push(rhs);
                Ok(n)
            },
            Some(Token::Slash) => {
                let mut n = Node::new(GrammarItem::Divide);
                self.next();
                let rhs = self.parse_factor()?;
                println!("parse_mult (/) : pushing {:?}", lhs);
                n.children.push(lhs);
                println!("parse_mult (/) : pushing {:?}", rhs);
                n.children.push(rhs);
                Ok(n)
            },
            // Some(_) => Err(format!("expected * or /")),
            _ => Ok(lhs),
        }
    }

    /// Parses a factor expression based on the following rule:
    /// 
    /// <factor> ::= '(' <expression> ')' | <atom> ;
    /// 
    fn parse_factor(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(Token::LeftParen) => {
                self.next();
                let expr = self.parse_addition();
                self.expect_rparen()?;
                return expr;
            },
            Some(Token::Integer(_)) => self.parse_atom(),
            _ => {
                println!("{}", self.cursor);
                Err(format!("error parsing factor"))
            },
        }
    }

    /// Parses a multiplication expression based on the following rule:
    /// 
    /// <atom> ::= <integer>
    /// 
    fn parse_atom(&mut self) -> Result<Node, String> {
        self.expect_integer()
    }

    fn expect_integer(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Integer(i)) => Ok(Node::new(GrammarItem::Integer(*i))),
            _ => Err(format!("expected integer")),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::RightParen) => Ok(()),
            _ => Err(format!("expected right parenthesis")),
        }
    }

    fn expect_semicolon(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::Semicolon) => Ok(()),
            _ => Err(format!("expected semicolon")),
        }
    }

    /// Next returns an optional token and advances the cursor if there is
    /// Some() next token.
    fn next(&mut self) -> Option<&Token> {
        if !self.is_done() {
            self.cursor += 1;
            return Some(&self.tokens[self.cursor - 1]);
        }
        None
    }

    /// Peek returns an optional token without advancing the cursor.
    pub fn peek(&self) -> Option<&Token> {
        if !self.is_done() {
            return Some(&self.tokens[self.cursor]);
        }
        None
    }

    //// Returns whether we are at the end of the token stream.
    pub fn is_done(&self) -> bool {
        self.cursor >= self.tokens.len()
    }
}