# parsley :herb:

Implementing the Pratt parsing algorithm in Rust. I decided to use the 
following BNF grammar which is also the proposed new syntax for my programming
language, [Ivy](https://github.com/gtr/ivy).

```html
<statement> ::= <expression> ';' ;

<expression> ::= <letExpr>      [ ]
               | <mutExpr>      [ ]
               | <fnExpr>       [x]
               | <ifExpr>       [x]
               | <pubExpr>      [x]
               | <dataExpr>     [x]
               | <structStmt>   [ ]
               | <packageStmt>  [x]
               | <importStmt>   [x]
               | <matchExpr>    [x]
               | <whileExpr>    [x]
               | <doExpr>       [x]
               | <returnExpr>   [x]
               | <or>           [ ]
               | <tupleAny> ;   [x]

<letExpr>       ::= 'let'['mut']? [<symbol>|<tupleSymbols>] ['::'<typeFn>]?'='<expression>;
<mutExpr>       ::= 'mut' [ <symbol> | <access> ] '=' <expression> ;
<fnExpr>        ::= <fnAnon> | <fnSignature> | <fnDeclaration> ;
<ifExpr>        ::= 'if' <expression> 'then' <expression> [ 'else' <expression> ]? ;
<pubExpr>       ::= 'pub' [ <fnSignature> | <fnDeclaration> | <typeExpr> | <structStmt> ] ;
<dataExpr>      ::= 'data' <symbol> [ <dataGenerics> ]? '(' <dataVariants> ')' ;
<structStmt>    ::= <structAnon> | <structDeclaration> ;
<packageStmt>   ::= 'package' <symbol> ;
<importStmt>    ::= 'import' [ <string> | <tupleStrings> ] ;
<matchExpr>     ::= 'match' <or> 'with' '(' [ <matchBranch> ]* ')' ;
<whileExpr>     ::= 'while' <or> '{' [ <statement> ]* '}' ;
<doExpr>        ::= 'do' '{' [ <statement> ]* '}' ;
<returnExpr>    ::= 'return' <expression> ;

<!-- [x] Functions -->
<fnAnon>        ::= 'fn' <fnArgs> [ ':' <typeFn> ]? '=>' <expression> ;
<fnSignature>   ::= 'fn' <symbol> '::' <typeFn> ;
<fnDeclaration> ::= 'fn' <symbol> <fnArgs> [ ':' <typeFn> ]? '=>' <expression> ;

<!-- [ ] Function Arguments -->
<fnArgs>        ::= '(' [ <fnArgsTyped> [ ',' <fnArgsTyped> ]* ]? ')' ;
<fnArgsTyped>   ::= <symbol> [ ':' <typeFn> ] ? ;

<!-- [x] Data Type Branches -->
<dataGenerics>  ::= '<' <symbol> [',' <symbol> ]* '>' ; ;
<dataVariants>  ::= [ '|' ]? <dataItem> [ '|' <dataItem> ]* ;
<dataItem>      ::= <symbol> [ '::' <typeFn> ]? ;

<!-- [ ] Structs -->
<structAnon>        ::= 'struct'          '(' <structFields> ')' ;
<structDeclaration> ::= 'struct' <symbol> '(' <structFields> ')' ;
<structFields>      ::= <structField> [ ',' <structField> ]* [ ',' ]? ;
<structField>       ::= <symbol> '::' <typeFn> ;
    
<!-- [x] Match Branches -->
<matchBranch>   ::= '|' <expression> '->' <expression>

<!-- [x] Type Literals -->
<typeFn>    ::= <typeCmpst> [ '->' <typeCmpst> ]? ;
<typeCmpst> ::= <typeLst> | <symbol> '<' [ <typeLst> [ ',' <typeLst> ]* ] '>' ;
<typeLst>   ::= <typeTuple> | '[' <typeFn> ']' ;
<typeTuple> ::= <type> | '(' <typeFn> [ ',' <typeFn> ]* ')' ;
<type>      ::= [ 'mut' ]? <symbol>  ;

<!-- [x] Binary & Unary Expressions, Operator Precedence -->
<or>            ::= <and> [ '||' <and> ]* ;
<and>           ::= <equality> [ '&&' <equality> ]* ;
<equality>      ::= <comparison> [ ( '==' | '!=' ) <comparison> ]* ;
<comparison>    ::= <addition> [ ( '>' | '>=' | '<' | '<=' ) <addition> ]* ;
<addition>      ::= <mult> [ ( '+' | '-' )  <mult> ]* ;
<mult>          ::= <unary>  [ ( '*' | '/' ) <unary> ]* ;
<unary>         ::= ( '!' | '-' ) <callExpr> | <callExpr> ;

<!-- [ ] Call & Access Expressions -->
<callExpr>      ::= <accessAttr> | <accessAttr> <tuple> ;
<accessAttr>    ::= <accessIndx> [ '.' <callExpr> ]* ;
<accessIndx>    ::= <factor> [ '[' <or> ']' ]* ;

<!-- [x] Factors & tuples -->
<factor>    ::= '(' [ <or> ]? ')' 
              | <tuple> 
              | <listExpr> 
              | <atom> ;
<tuple>     ::= '(' <expression> [ ',' <expression> ]* ')' ;

<!-- [x] List Literals -->
<listExpr>      ::= <listSplit> | <listLiteral> ;
<listLiteral>   ::= '[' [ <listItems> ]? ']' ;
<listSplit>     ::= '[' <symbol> '|' <symbol> ']' ;
<listItems>     ::= <expression> [ ',' <expression> ]* ;

<!-- [x] Tuples -->
<tupleAny>      ::= '(' <expression> [ ','  <expression> ]* ')' ;
<tupleSymbols>  ::= '(' <symbol> [ ',' <symbol> ]* [ ',' ]? ')' ;
<tupleStrings>  ::= '(' <string> [ ',' <string> ]* [ ',' ]? ')' ;
    
<!-- [x] Atoms -->
<atom>  ::= <integer>
          | <symbol> 
          | <string> ;
```

## Example

The following Ivy code: 

```haskell
fn factorial :: Int -> Int;
fn factorial (0) => 1;
fn factorial (n) => n * factorial(n - 1);
```

Is equivalent to the following vector of tokens:
```rust
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
```

And results in the following syntax tree:

```
[root]
  0: [fn signature]
    name: [Symbol 'factorial']
    type: [->]
      lhs: [Symbol 'Int']
      rhs: [Symbol 'Int']
  1: [fn declaration]
    name: [Symbol 'factorial']
    args: [Int '0']
    value: [Int '1']
  2: [fn declaration]
    name: [Symbol 'factorial']
    args: [Symbol 'n']
    value: [*]
      lhs: [Symbol 'n']
      rhs: [call]
        lhs: [Symbol 'factorial']
        arg: [-]
            lhs: [Symbol 'n']
            rhs: [Int '1']
```