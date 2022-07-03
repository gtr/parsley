# parsley :herb:

Implementing the Pratt parsing algorithm in Rust. I decided to use the 
following BNF grammar which is also the proposed new syntax for my programming
language, [Ivy](https://github.com/gtr/ivy).

```html
<statement> ::= <expression> ';' ;

<expresion> ::= <letExpr> 
              | <funcExpr>
              | <ifExpr>
              | <or> ;

<letExpr>   ::= 'let' [ <symbol> | <tupleExpr> ] '=' <expression> ;
<funcExpr>  ::= 'fn' <tupleExpr> '->' <expression> ;
<ifExpr>    ::= 'if' '(' <or> ')' 'then' <expression> [ 'else' <expression> ]? ;

<tupleExpr> ::= '(' [ <commas> ]? ')' ;
<commas>    ::= <atom> [ ',' <atom> ]* ;

<or>            ::= <and> [ '||' <and> ]* ;
<and>           ::= <equality> [ '&&' <equality> ]* ;
<equality>      ::= <comparison> [ [ '==' | '!=' ] <comparison> ]* ;
<comparison>    ::= <addition> [ [ '>' | '>=' | '<' | '<=' ] <addition> ]* ;
<addition>      ::= <mult> [ ( '+' | '-' ) <mult> ]* ;
<mult>          ::= <unary> [ ( '*' | '/' ) <unary> ]* ;
<unary>         ::= [ '!' | '-' | '++' | '--' ] <call> | <call> ;
<call>          ::= <factor> <tupleExpr> | <factor> ;
<factor>        ::= '(' <or> ')' | <atom> ;

<atom> ::= <integer>
         | <symbol> 
         | <boolean> ;
```

### example

The following code
```rust
// let add = fn(a, b) -> a + b;
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

fn main() {
    let tokens = test_6();

    let mut p = Parser::new(tokens);
    let root = p.parse().unwrap();

    print_tree(&root);
}
```

produces:
```
1: let
2:   Symbol: add
2:   fn
3:     tuple
4:       Symbol: a
4:       Symbol: b
3:     +
4:       Symbol: a
4:       Symbol: b
```

