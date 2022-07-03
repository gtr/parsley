# parsley

Implementing the Pratt parsing algorithm in Rust. I decided to use the 
following BNF grammar which is also the proposed new syntax for my programming
language, [Ivy](https://github.com/gtr/ivy).

```html
<expression>    ::= <addition> ';' ;
<addition>      ::= <mult> [ ( '+' | '-' ) <mult> ]* ;
<mult>          ::= <factor> [ ( '*' | '/' ) <factor> ]* ;
<factor>        ::= '(' <expression> ')' | <atom> ;

<atom> ::= <integer>
         | <string> ;
```