use crate::tokens::tokens::*;
use crate::parser::ast::*;

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Parser {
        for token in &tokens {
            print!("{} ", token);
        }
        println!("");
        Parser{
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let mut root = Node::new(GrammarItem::Root);

        while !self.is_done() {
            let expr = self.parse_statement()?;
            root.children.push(expr);
        }

        Ok(root)
    }

    /// Parses a statement based on the following rule:
    /// 
    /// <statement> ::= <expression> ';' ;
    /// 
    fn parse_statement(&mut self) -> Result<Node, String> {
        let expr = self.parse_expression()?;
        self.expect_semicolon()?;

        Ok(expr)
    }

    /// Parses an expression based on the following rule:
    /// 
    /// <expresion> ::= <letExpr> 
    ///               | <funcExpr>
    ///               | <ifExpr>
    ///               | <or> ;
    /// 
    fn parse_expression(&mut self) -> Result<Node, String> {
        let expr = match self.peek() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::Fn) => self.parse_function(),
            Some(Token::If) => self.parse_if(),
            _ => self.parse_or(),
        };

        expr
    }


    /// Parses a let expression based on the following rule:
    /// 
    /// <letExpr> ::= 'let' [ <symbol> | <tupleExpr> ] '=' <expression> ;
    /// 
    fn parse_let(&mut self) -> Result<Node, String> {
        let mut let_node = self.expect_let()?;

        match self.peek() {
            Some(Token::LeftParen) => {
                let_node.children.push(self.parse_tuple()?);
            },
            Some(Token::Symbol(_)) => {
                let_node.children.push(self.expect_symbol()?);
            },
            _ => {
                return Err(format!("expected symbol or tuple"))
            },
        }
        
        self.expect_bind()?;

        let_node.children.push(self.parse_expression()?);

        Ok(let_node)
    }

    /// Parses a function expression based on the following rule:
    /// 
    /// <funcExpr>  ::= 'fn' <tupleExpr> '->' <expression> ;
    /// 
    fn parse_function(&mut self) -> Result<Node, String> {
        let mut func = Node::new(GrammarItem::Fn);

        self.expect_fn()?;
        func.children.push(self.parse_tuple()?);
        self.expect_arrow()?;
        func.children.push(self.parse_expression()?);

        Ok(func)
    }

    /// Parses an if expression based on the following rule:
    /// 
    /// <ifExpr> ::= 'if' '(' <or> ')' 
    ///             'then' <expression> 
    ///             [ 'else' <expression> ]? ;
    /// 
    fn parse_if(&mut self) -> Result<Node, String> {
        let mut if_node = self.expect_if()?;

        self.expect_lparen()?;
        if_node.children.push(self.parse_or()?);
        self.expect_rparen()?;

        match self.peek() {
            Some(Token::Then) => {
                self.expect_then()?;
                if_node.children.push(self.parse_or()?);
                match self.peek() {
                    Some(Token::Else) => {
                        self.expect_else()?;
                        if_node.children.push(self.parse_or()?);
                    },
                    _ => {return Ok(if_node);}
                }
            },
            _ => return Err(format!("error parsing if expression")),
        };

        Ok(if_node)
    }

    /// Parses a tuple expression based on the following rule:
    /// 
    /// <tupleExpr> ::= '(' [ <commas> ]? ')' ;
    /// 
    fn parse_tuple(&mut self) -> Result<Node, String> {
        let mut tuple = Node::new(GrammarItem::Tuple);
        
        self.expect_lparen()?;

        match self.peek() {
            Some(Token::RightParen) => {
                self.expect_rparen()?;
                return Ok(tuple);
            },
            _ => {
                let commas = self.parse_commas()?;
                for n in commas {
                    tuple.children.push(n);
                }
                self.expect_rparen()?;
            },
        };


        Ok(tuple)
    }

    /// Parses a comma expression based on the following rule:
    /// 
    /// <commas> ::= <atom> [ ',' <atom> ]* ;
    /// 
    /// Note: this function returns a vector of tokens rather than just a 
    /// single token.
    fn parse_commas(&mut self) -> Result<Vec<Node>, String> {
        let mut atoms = Vec::new();

        let first = self.parse_atom()?;
        atoms.push(first);

        loop {
            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                    atoms.push(self.parse_atom()?);
                }
                _ => break,
            }
        }

        Ok(atoms)
    }

    /// Parses an and expression based on the following rule:
    /// 
    /// <or> ::= <and> [ '||' <and> ]* ;
    /// 
    fn parse_or(&mut self) -> Result<Node, String> {
        let lhs = self.parse_and()?;

        let mut and = match self.peek() {
            Some(Token::Or) => self.expect_or()?,
            _ => return Ok(lhs),
        };

        let rhs = self.parse_and()?;

        and.children.push(lhs);
        and.children.push(rhs);

        Ok(and)
    }


    /// Parses an and expression based on the following rule:
    /// 
    /// <and> ::= <equality> [ '&&' <equality> ]* ;
    /// 
    fn parse_and(&mut self) -> Result<Node, String> {
        let lhs = self.parse_equality()?;

        let mut and = match self.peek() {
            Some(Token::And) => self.expect_and()?,
            _ => return Ok(lhs),
        };

        let rhs = self.parse_equality()?;

        and.children.push(lhs);
        and.children.push(rhs);

        Ok(and)
    }

    /// Parses an equality expression based on the following rule:
    /// 
    /// <equality>      ::= <comparison> [ [ '==' | '!=' ] <comparison> ]* ;
    /// 
    fn parse_equality(&mut self) -> Result<Node, String> {
        let lhs = self.parse_comparison()?;

        let mut equality = match self.peek() {
            Some(Token::Eq) => self.expect_eq()?,
            Some(Token::NotEq) => self.expect_noteq()?,
            _ => return Ok(lhs),
        };

        let rhs = self.parse_comparison()?;

        equality.children.push(lhs);
        equality.children.push(rhs);

        Ok(equality)
    }

    /// Parses an addition expression based on the following rule:
    /// 
    /// <comparison> ::= <addition> [ [ '>' 
    ///                              | '>=' 
    ///                              | '<' 
    ///                              | '<=' ] <addition> ]* ;
    /// 
    fn parse_comparison(&mut self) -> Result<Node, String> {
        let lhs = self.parse_addition()?;

        let mut comparison = match self.peek() {
            Some(Token::Greater) => self.expect_greater()?,
            Some(Token::GreaterEqual) => self.expect_greater_equal()?,
            Some(Token::Less) => self.expect_less()?,
            Some(Token::LessEqual) => self.expect_less_equal()?,
            _ => return Ok(lhs),
        };

        let rhs = self.parse_addition()?;

        comparison.children.push(lhs);
        comparison.children.push(rhs);

        Ok(comparison)
    }

    /// Parses an addition expression based on the following rule:
    /// 
    /// <addition> ::= <mult> [ ( '+' | '-' ) <mult> ]* ;
    /// 
    fn parse_addition(&mut self) -> Result<Node, String> {
        let lhs = self.parse_mult()?;

        let mut addition = match self.peek() {
            Some(Token::Plus) => self.expect_plus()?,
            Some(Token::Minus) => self.expect_minus()?,
            _ => return Ok(lhs),
        };

        let rhs = self.parse_mult()?;

        addition.children.push(lhs);
        addition.children.push(rhs);

        Ok(addition)
    }

    /// Parses a multiplication expression based on the following rule:
    /// 
    /// <mult> ::= <unary> [ ( '*' | '/' ) <unary> ]* ;
    /// 
    fn parse_mult(&mut self) -> Result<Node, String> {
        let lhs = self.parse_unary()?;

        let mut addition = match self.peek() {
            Some(Token::Star) => self.expect_mult()?,
            Some(Token::Slash) => self.expect_div()?,
            _ => return Ok(lhs),
        };

        let rhs = self.parse_unary()?;

        addition.children.push(lhs);
        addition.children.push(rhs);

        Ok(addition)
    }

    /// Parses a unary expression based on the following rule:
    /// 
    /// <unary> ::= [ '!' | '-' | '++' | '--' ] <call> | <call> ;
    /// 
    fn parse_unary(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(Token::Not) | Some(Token::Minus) 
            | Some(Token::PlusPlus) | Some(Token::MinusMimus) => {
                let mut op = self.expect_unary()?;
                op.children.push(self.parse_call()?);
                Ok(op)
            }
            _ => self.parse_call(),
        }
    }

    /// Parses a call expression based on the following rule:
    /// 
    /// <call> ::= <factor> '(' [ <commas> ]? ')' | <factor> ;
    /// 
    fn parse_call(&mut self) -> Result<Node, String> {
        let mut lhs = self.parse_factor()?;

        match self.peek() {
            Some(Token::LeftParen) => {
                lhs.children.push(self.parse_tuple()?);
                return Ok(lhs);
            }
            _ => return Ok(lhs),
        }
        
    }

    /// Parses a factor expression based on the following rule:
    /// 
    /// <factor> ::= '(' <or> ')' | <atom> ;
    /// 
    fn parse_factor(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(Token::LeftParen) => {
                self.expect_lparen()?;
                let expr = self.parse_or();
                self.expect_rparen()?;
                expr
            },
            Some(Token::Integer(_)) | Some(Token::Symbol(_)) 
            | Some(Token::True) | Some(Token::False) => self.parse_atom(),
            _ => Err(format!("error parsing factor")),
        }
    }

    /// Parses an atom expression based on the following rule:
    /// 
    /// <atom> ::= <integer>
    ///          | <symbol> 
    ///          | <boolean> ;
    /// 
    fn parse_atom(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(Token::Integer(_)) => self.expect_integer(),
            Some(Token::Symbol(_)) => self.expect_symbol(),
            Some(Token::True) | Some(Token::False) => self.expect_boolean(),
            _ => Err(format!("expected an atom")),
        }
    }

    fn expect_let(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Let) => Ok(Node::new(GrammarItem::Let)),
            _ => Err(format!("expected let")),
        }
    }
    
    fn expect_integer(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Integer(i)) => Ok(Node::new(GrammarItem::Integer(*i))),
            _ => Err(format!("expected integer")),
        }
    }

    fn expect_symbol(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Symbol(s)) => Ok(Node::new(GrammarItem::Symbol(s.to_string()))),
            _ => Err(format!("expected symbol")),
        }
    }

    fn expect_boolean(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::True) => Ok(Node::new(GrammarItem::True)),
            Some(Token::False) => Ok(Node::new(GrammarItem::False)),
            _ => Err(format!("expected boolean")),
        }
    }

    fn expect_plus(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Plus) => Ok(Node::new(GrammarItem::Plus)),
            _ => Err(format!("expected plus")),
        }
    }

    fn expect_greater(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Greater) => Ok(Node::new(GrammarItem::Greater)),
            _ => Err(format!("expected greater")),
        }
    }

    fn expect_greater_equal(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::GreaterEqual) => Ok(Node::new(GrammarItem::GreaterEqual)),
            _ => Err(format!("expected greater equal")),
        }
    }

    fn expect_less(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Less) => Ok(Node::new(GrammarItem::Less)),
            _ => Err(format!("expected less")),
        }
    }

    fn expect_less_equal(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::LessEqual) => Ok(Node::new(GrammarItem::LessEqual)),
            _ => Err(format!("expected less equal")),
        }
    }

    fn expect_eq(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Eq) => Ok(Node::new(GrammarItem::Eq)),
            _ => Err(format!("expected equal")),
        }
    }

    fn expect_noteq(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::NotEq) => Ok(Node::new(GrammarItem::NotEq)),
            _ => Err(format!("expected not equal")),
        }
    }

    fn expect_unary(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Not) => Ok(Node::new(GrammarItem::Not)),
            Some(Token::Minus) => Ok(Node::new(GrammarItem::Minus)),
            Some(Token::MinusMimus) => Ok(Node::new(GrammarItem::MinusMimus)),
            Some(Token::PlusPlus) => Ok(Node::new(GrammarItem::PlusPlus)),
            _ => Err(format!("expected unary")),
        }
    }

    fn expect_minus(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Fn) => Ok(Node::new(GrammarItem::Minus)),
            _ => Err(format!("expected minus")),
        }
    }
    
    fn expect_mult(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Star) => Ok(Node::new(GrammarItem::Multiply)),
            _ => Err(format!("expected star")),
        }
    }

    fn expect_div(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Fn) => Ok(Node::new(GrammarItem::Divide)),
            _ => Err(format!("expected fn")),
        }
    }

    fn expect_and(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::And) => Ok(Node::new(GrammarItem::And)),
            _ => Err(format!("expected and")),
        }
    }

    fn expect_or(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::Or) => Ok(Node::new(GrammarItem::Or)),
            _ => Err(format!("expected or")),
        }
    }

    fn expect_if(&mut self) -> Result<Node, String> {
        match self.next() {
            Some(Token::If) => Ok(Node::new(GrammarItem::If)),
            _ => Err(format!("expected if")),
        }
    }

    fn expect_then(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::Then) => Ok(()),
            _ => Err(format!("expected then")),
        }
    }

    fn expect_else(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::Else) => Ok(()),
            _ => Err(format!("expected else")),
        }
    }

    fn expect_fn(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::Fn) => Ok(()),
            _ => Err(format!("expected fn")),
        }
    }

    fn expect_lparen(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::LeftParen) => Ok(()),
            _ => Err(format!("expected left parenthesis")),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::RightParen) => Ok(()),
            _ => Err(format!("expected right parenthesis")),
        }
    }

    fn expect_bind(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::Bind) => Ok(()),
            _ => Err(format!("expected =")),
        }
    }

    fn expect_arrow(&mut self) -> Result<(), String> {
        match self.next() {
            Some(Token::Arrow) => Ok(()),
            _ => Err(format!("expected arrow")),
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
