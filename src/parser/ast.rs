#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

const TAB: &str = "  ";

use crate::Token;
use crate::TokenType;

// ====================================
// Let Node
// ====================================

// <letExpr>  ::= 'let' [ 'mut' ]? [ <symbol> | <tupleSymbols> ] [ '::' <typeFn> ]? '=' <expression> ;
pub struct LetExpr {
    pub symbols: Vec<Node>,
    pub rhs: Box<Node>,
    pub token: Box<Token>,
    pub is_mut: bool,
    pub ttype: Box<Option<Node>>
}

pub fn NewLetExpr(tok: Token, symbols: Vec<Node>, rhs: Node, ttype: Option<Node>) -> Node {
    Node::LetExpr(LetExpr { 
        symbols: symbols, 
        rhs: Box::new(rhs), 
        token: Box::new(tok), 
        is_mut: false, 
        ttype: Box::new(ttype)
    })
}

pub fn NewLetMutExpr(tok: Token, symbols: Vec<Node>, rhs: Node, ttype: Option<Node>) -> Node {
    Node::LetExpr(LetExpr { 
        symbols: symbols, 
        rhs: Box::new(rhs), 
        token: Box::new(tok),
        is_mut: true, 
        ttype: Box::new(ttype)
    })
}

// ====================================
// Mut Node
// ====================================

// <mutExpr> ::= 'mut' [ <symbol> | <access> ] '=' <expression> ;
pub struct MutExpr {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewMutExpr(tok: Token, lhs: Node, rhs: Node) -> Node {
    Node::MutExpr(MutExpr { 
        lhs: Box::new(lhs), rhs: Box::new(rhs), token: Box::new(tok) 
    })
}

// ====================================
// Function Expressions
// ====================================

// <fnAnon>  ::= 'fn' <fnArgs> [ ':' <typeFn> ]? '=>' <expression> ;
pub struct FnAnon {
    pub arguments: Vec<Node>,
    pub type_out: Box<Option<Node>>,
    pub rhs: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewFnAnon(tok: Token, arguments: Vec<Node>, type_out: Option<Node>, rhs: Node) -> Node {
    Node::FnAnon(FnAnon { 
        arguments: arguments, 
        type_out: Box::new(type_out),
        rhs: Box::new(rhs), 
        token: Box::new(tok), 
    })
}

// <fnSignature> ::= 'fn' <symbol> '::' <typeFn> ;
pub struct FnSignature {
    pub symbol: Box<Node>,
    pub ttype: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewFnSignature(tok: Token, symbol: Node, ttype: Node) -> Node {
    Node::FnSignature(FnSignature { 
        token: Box::new(tok), symbol: Box::new(symbol), ttype: Box::new(ttype),
    })
}

// <fnDeclaration> ::= 'fn' <symbol> <fnArgs> [ ':' <typeFn> ]? '=>' <expression> ;
pub struct FnDeclaration {
    pub symbol: Box<Node>,
    pub arguments: Vec<Node>,
    pub type_out: Box<Option<Node>>,
    pub rhs: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewFnDeclaration(
    tok: Token, symbol: Node, rhs: Node,
    arguments: Vec<Node>, type_out: Option<Node>, 
) -> Node {
    Node::FnDeclaration(FnDeclaration { 
        symbol: Box::new(symbol), 
        arguments: arguments, 
        type_out: Box::new(type_out), 
        rhs: Box::new(rhs), 
        token: Box::new(tok),
    })
}

// ====================================
// Function Arguments
// ====================================


// <fnArgsTyped> ::= <symbol> [ ':' <typeFn> ] ? ;
pub struct FnArgTyped {
    pub symbol: Box<Node>,
    pub ttype: Box<Option<Node>>,
}

pub fn NewFnArgTyped(symbol: Node, ttype: Option<Node>) -> Node {
    Node::FnArgTyped ( FnArgTyped{
        symbol: Box::new(symbol), ttype: Box::new(ttype),
    })
}

// ====================================
// If Node
// ====================================

// <ifExpr> ::= 'if' <or> 'then' <expression> [ 'else' <expression> ]? ;
pub struct IfExpr {
    pub cond: Box<Node>,
    pub true_branch: Box<Node>,
    pub false_branch: Box<Option<Node>>,
    pub token: Box<Token>,
}

pub fn NewIfExpr(
    tok: Token, cond: Node,  true_branch: Node, false_branch: Option<Node>
) -> Node {
    Node::IfExpr(IfExpr {
        cond: Box::new(cond),
        true_branch: Box::new(true_branch),
        false_branch: Box::new(false_branch),
        token: Box::new(tok),
    })
}

// ====================================
// Pub Expression
// ===================================

// <pubExpr> ::= 'pub' [ <fnSignature> | <fnDeclaration> | <typeExpr> | <structStmt> ] ;
pub struct PubExpr {
    pub rhs: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewPubExpr (tok: Token, rhs: Node) -> Node {
    Node::PubExpr( PubExpr{ token: Box::new(tok), rhs: Box::new(rhs) })
}

// ====================================
// Data Types 
// ====================================

// <dataDeclaration> ::= 'data' <symbol> '(' <typeVariants> ')' ;
pub struct DataDeclaration {
    pub symbol: Box<Node>,
    pub generics: Vec<Node>,
    pub variants: Vec<Node>,
    pub token: Box<Token>,
}

pub fn NewDataDelcaration(tok: Token, symbol: Node,  generics: Vec<Node>, variants: Vec<Node>) -> Node {
    Node::DataDeclaration(DataDeclaration {
        token: Box::new(tok), symbol: Box::new(symbol), generics, variants
    })
}

// <dataVariants>  ::= [ '|' ]? <dataItem> [ '|' <dataItem> ]* ;
// <dataItem>      ::= <symbol> [ '::' <typeFn> ]? ;
pub struct DataItem {
    pub symbol: Box<Node>,
    pub ttype: Box<Node>,
}

pub fn NewDataItem(symbol: Node, ttype: Node) -> Node {
    Node::DataItem(DataItem { 
        symbol: Box::new(symbol), ttype: Box::new(ttype)
    })
}


// ====================================
// Structs
// ====================================

// <structAnon> ::= 'struct' '(' <stuctFields> ')' ;
pub struct StructAnon {
    pub fields: Vec<Node>,
    pub token: Box<Token>,
}

pub fn NewStructAnon(tok: Token, fields: Vec<Node>) -> Node {
    Node::StructAnon(StructAnon { token: Box::new(tok), fields })
}

// <structDeclaration> ::= 'struct' <symbol> '(' <stuctFields> ')' ;
pub struct StructDeclaration {
    pub symbol: Box<Node>,
    pub fields: Vec<Node>,
    pub token: Box<Token>,
}

pub fn NewStructDeclaration(tok: Token, symbol: Node, fields: Vec<Node>) -> Node {
    Node::StructDeclaration(StructDeclaration { 
        symbol: Box::new(symbol), token: Box::new(tok), fields 
    })
}

// <structFields>      ::= <strictField> [ ',' <strictField> ]* [ ',' ]? ;
// <structField>       ::= <symbol> '::' <typeFn> ;
pub struct StructField {
    pub symbol: Box<Node>,
    pub ttype: Box<Node>,
}

pub fn NewStructField(symbol: Node, ttype: Node) -> Node {
    Node::StructField(
        StructField{ symbol: Box::new(symbol), ttype: Box::new(ttype)} 
    )
}

// ====================================
// Package Statement
// ====================================

// <packageStmt>   ::= 'package' <symbol> ;
pub struct Package {
    pub token: Box<Token>,
    pub rhs: Box<Node>,
}

pub fn NewPackage (rhs: Node, token: Token) -> Node {
    Node::Package(Package { token: Box::new(token), rhs: Box::new(rhs) })
}

// ====================================
// Import Statement
// ====================================

// <importStmt>    ::= 'import' [ <string> | <tupleStrings> ] ;
pub struct Import {
    pub token: Box<Token>,
    pub rhs: Vec<Node>,
}

pub fn NewImport (rhs: Vec<Node>, token: Token) -> Node {
    Node::Import(Import { token: Box::new(token), rhs })
}

// ====================================
// Match Expressions
// ====================================

// <matchExpr>     ::= 'match' <or> '(' [ <matchBranch> ]* ')' ;
pub struct MatchExpression {
    pub lhs: Box<Node>,
    pub branches: Vec<Node>,
    pub token: Box<Token>,
}

pub fn NewMatchExpression(tok: Token, lhs: Node, branches: Vec<Node>) -> Node {
    Node::MatchExpression(
        MatchExpression { token: Box::new(tok), lhs: Box::new(lhs), branches 
    })
}

// <matchBranch>   ::= '|' <expression> '->' <expression>
pub struct MatchBranch {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewMatchBranch(tok: Token, lhs: Node, rhs: Node) -> Node {
    Node::MatchBranch(MatchBranch { 
        token: Box::new(tok), lhs: Box::new(lhs), rhs: Box::new(rhs) 
    })
}

// ====================================
// List Expressions
// ====================================

// <listLiteral>   ::= '[' [ <listItems> ]? ']' ;
// <listItems>     ::= <expression> [ ',' <expression> ]* ;
pub struct ListLiteral {
    pub items: Vec<Node>,
    pub token: Box<Token>,
}

pub fn NewListExpression(tok: Token, items: Vec<Node>) -> Node {
    Node::ListLiteral(ListLiteral { token: Box::new(tok), items })
}

// <listSplit>     ::= '[ <symbol> '|' <symbol> ']' ;
pub struct ListSplit {
    pub head: Box<Node>,
    pub tail: Box<Node>,
}

pub fn NewListSplit(head: Node, tail: Node) -> Node {
    Node::ListSplit(ListSplit {head: Box::new(head), tail: Box::new(tail)})
}

// ====================================
// Type Literals
// ====================================

// <typeFn>    ::= <typeLst>   | <typeLst> '->' <typeLst> ;
pub struct TypeFn {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub token: Box<Token>
}

pub fn NewTypeFn(lhs: Node, rhs: Node, token: Token) -> Node {
    Node::TypeFn(TypeFn{ 
        lhs: Box::new(lhs), 
        rhs: Box::new(rhs), 
        token: Box::new(token)
    })
}

// <typeLst>   ::= <typeTuple> | '[' <typeFn> ']' ;
pub struct TypeLst {
    pub ttype: Box<Node>,
    pub token: Box<Token>
}

pub fn NewTypeLst(ttype: Node, token: Token) -> Node { 
    Node::TypeLst(TypeLst { ttype: Box::new(ttype), token: Box::new(token) }) 
}

// <typeTuple> ::= <typeCmpst> | '(' <typeFn> [ ',' <typeFn> ]* ')' ;
pub struct TypeTuple {
    pub ttypes: Vec<Node>,
}

pub fn NewTypeTuple(ttypes: Vec<Node>) -> Node { Node::TypeTuple(TypeTuple { ttypes }) }

// <typeCmpst> ::= <type>      | <symbol> '<' [ <typeFn> [ ',' <typeFn> ]* ] '>' ;
pub struct TypeCmpst {
    pub ttype: Box<Node>,
    pub items: Vec<Node>,
}

pub fn NewTypeCmpst(ttype: Node, items: Vec<Node>) -> Node {
    Node::TypeCmpst(TypeCmpst{ ttype: Box::new(ttype), items })
}

// <type>      ::= [ 'mut' ]? <symbol> | <typeFn> ;
pub struct Ttype {
    pub symbol: Box<Node>,
    pub is_mut: bool,
}

pub fn NewTtype(symbol: Node, is_mut: bool) -> Node {
    Node::Ttype(Ttype { symbol: Box::new(symbol), is_mut })
}


// ====================================
// While Expressions
// ====================================

// <whileExpr>     ::= 'while' <or> '{' [ <statement> ]* '}' ;
pub struct WhileExpression {
    pub cond: Box<Node>,
    pub statements: Vec<Node>,
    pub token: Box<Token>,
}

pub fn NewWhileExpression(tok: Token, cond: Node, statements: Vec<Node>) -> Node {
    Node::WhileExpression(WhileExpression { 
        token: Box::new(tok), cond: Box::new(cond), statements 
    })
}

// ====================================
// Do Expressions
// ====================================

// <doExpr>        ::= 'do' '{' [ <statement> ]* '}' ;
pub struct DoExpression {
    pub token: Box<Token>,
    pub statements: Vec<Node>,
}

pub fn NewDoExpression(tok: Token, statements: Vec<Node> ) -> Node {
    Node::DoExpression(DoExpression { token: Box::new(tok), statements })
}

// ====================================
// Return Expressions
// ====================================

// <returnExpr>    ::= 'return' <expression> ;
pub struct ReturnExpression {
    pub value: Box<Node>,
    pub token: Box<Token>,
}

pub fn NewReturnExpression(tok: Token, value: Node) -> Node {
    Node::ReturnExpression(ReturnExpression {
        token: Box::new(tok), value: Box::new(value)
    })
}

// ====================================
// Binary & Unary Expressions
// ====================================

pub struct BinaryExpression {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub token: Token,
}

pub fn NewBinaryExpression(tok: Token, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpression(BinaryExpression {
        token: tok, lhs: Box::new(lhs), rhs: Box::new(rhs)
    })
}

pub fn copy_token(tok: &Token) -> Token {
    Token {
        col: tok.col, row: tok.row, typ: tok.typ.clone()
    }
}

pub struct UnaryExpression {
    pub rhs: Box<Node>,
    pub token: Token,
}

pub fn NewUnaryExpression(tok: Token, rhs: Node) -> Node {
    Node::UnaryExpression(UnaryExpression { token: tok, rhs: Box::new(rhs) })
}

// ====================================
// Calls
// ====================================

// <call> ::= <access> | <symbol> <tupleAny> ;
pub struct Call {
    pub lhs: Box<Node>,
    pub args: Vec<Node>,
}

pub fn NewCall(lhs: Node, args: Vec<Node>) -> Node { 
    Node::Call(Call{ lhs: Box::new(lhs), args }) 
}

// ====================================
// Access
// ====================================

// <access> ::= <symbol> '.' <symbol>
//            | <symbol> '.' <access>
//            | <symbol> '.' <call>
pub struct Access {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

pub fn NewAccess(lhs: Node, rhs: Node) -> Node {
    Node::Access(Access{ lhs: Box::new(lhs), rhs: Box::new(rhs) })
}

// <access> ::= <symbol> '[' <expression> ']' 
pub struct AccessIndex {
    pub symbol: Box<Node>,
    pub index: Box<Node>,
}

pub fn NewAccessIndex(symbol: Node, index: Node) -> Node {
    Node::AccessIndex(AccessIndex {
        symbol: Box::new(symbol),
        index: Box::new(index)
    })
}

// ====================================
// Tuples
// ====================================

// <tupleAny>      ::= '(' <expression> [ ','  <expression> ]* ')' ;
pub struct TupleAny {
    pub items: Vec<Node>,
}

pub fn NewTupleAny(items: Vec<Node>) -> Node {
    Node::TupleAny( TupleAny{ items })
}

// <tupleSymbols>  ::= '(' <symbol> [ ',' <symbol> ]* [ ',' ]? ')' ;
pub struct TupleSymbols {
    pub items: Vec<Node>,
}

pub fn NewTupleSymbols(items: Vec<Node>) -> Node {
    Node::TupleSymbols( TupleSymbols{ items })
}

// <tupleStrings>  ::= '(' <string> [ ',' <string> ]* [ ',' ]? ')' ;
pub struct TupleString {
    pub items: Vec<Node>,
}

pub fn NewTupleString(items: Vec<Node>) -> Node {
    Node::TupleString( TupleString{ items })
}

// ====================================
// Atoms
// ====================================

pub struct Atom {
    pub token: Token,
}

pub fn NewAtom(tok: Token) -> Node { 
    Node::Atom(Atom {token: tok }) 
}

pub struct Root {
    children: Vec<Node>,
}

pub fn NewRootNode(children: Vec<Node>) -> Node {
    Node::Root( Root { children })
}

pub enum Node {
    Root(Root),

    // [x] Let Expressions
    LetExpr(LetExpr),

    // [x] Mut Expressions
    MutExpr(MutExpr),
    
    // [x] Function Expressions
    FnAnon(FnAnon),
    FnSignature(FnSignature),
    FnDeclaration(FnDeclaration),
    
    // [x] If Expressions
    IfExpr(IfExpr),

    // [x] Public Expression
    PubExpr(PubExpr),
    
    // [ ] Data Type Expression
    DataDeclaration(DataDeclaration),
    DataItem(DataItem),
    
    // [ ] Structs
    StructAnon(StructAnon),
    StructDeclaration(StructDeclaration),
    StructField(StructField),

    // [ ] Import & Package
    Package(Package),
    Import(Import),

    // [ ] Match Expressions
    MatchExpression(MatchExpression),
    MatchBranch(MatchBranch),
    
    // [ ] List Expressions
    ListLiteral(ListLiteral),
    ListSplit(ListSplit),

    // [ ] While Expressions 
    WhileExpression(WhileExpression),

    // [ ] Do Expressions
    DoExpression(DoExpression),

    // [ ] Return Expressions
    ReturnExpression(ReturnExpression),

    // [ ] Type Literals
    TypeFn(TypeFn),
    TypeLst(TypeLst),
    TypeTuple(TypeTuple),
    TypeCmpst(TypeCmpst),
    Ttype(Ttype),
    FnArgTyped(FnArgTyped),
    
    // [ ] Binary & Unary Expressions
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    
    // [ ] Call & Access Expressions
    Call(Call),
    Access(Access),
    AccessIndex(AccessIndex),

    // [ ] Tuples
    TupleAny(TupleAny),
    TupleSymbols(TupleSymbols),
    TupleString(TupleString),
    
    // [x] Atoms
    Atom(Atom),
}

pub fn print_tree(node: Node) {
    print_tree_helper(node, 1);
}

fn print_tree_helper(node: Node, tabs: usize) {
    let indent = TAB.repeat(tabs);
    
    match node {
        Node::Root(node) => {
            println!("\n[root]");
            print_tuple(node.children, tabs);
        }
        Node::LetExpr(node) => {
            println!("[let{}]", if node.is_mut {" mut"} else {""});
            
            print!  ("{indent}lhs: ");
            print_tuple_single(node.symbols, tabs + 1);

            match *node.ttype {
                Some(node2) => {
                    print!  ("{indent}type: ");
                    print_tree_helper(node2, tabs + 1);
                },
                None => {}
            };
        
            print!("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::MutExpr(node) => {
            println!("[mut]");
            print!  ("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            print!  ("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        }
        Node::FnAnon(node) => {
            println!("[fn anon]");
            print!  ("{indent}args: ");
            print_tuple_single(node.arguments, tabs + 1);
            match *node.type_out {
                Some(node2) => {
                    print!  ("{}rtype: ", indent);
                    print_tree_helper(node2, tabs + 1);
                },
                None => {},
            };
            print!  ("{indent}value: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::FnSignature(node) => {
            println!("[fn signature]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.symbol, tabs + 1);
            print!  ("{indent}type: ");
            print_tree_helper(*node.ttype, tabs + 1);
        },
        Node::FnDeclaration(node) => {
            println!("[fn declaration]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.symbol, tabs + 1);
            if node.arguments.len() > 0 {
                print!  ("{indent}args: ");
                print_tuple_single(node.arguments, tabs + 1);
            }

            match *node.type_out {
                Some(node2) => {
                    print!  ("{}rtype: ", indent);
                    print_tree_helper(node2, tabs + 1);
                }
                None => {}
            }
            print!  ("{indent}value: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::IfExpr(node) => {
            println!("[if]");
            print!  ("{indent}cond: ");
            print_tree_helper(*node.cond, tabs + 1);
            print!  ("{indent}true branch: ");
            print_tree_helper(*node.true_branch, tabs + 1);
            match *node.false_branch {
                Some(node2) => {
                    print!  ("{indent}false branch: ");
                    print_tree_helper(node2, tabs + 1);
                }
                None => {}
            }
        },
        Node::PubExpr(node) => {
            println!("[pub]");
            print!  ("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::DataDeclaration(node) => {
            println!("[data decleration]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.symbol, tabs + 1);
            if node.generics.len() > 0 {
                print!  ("{indent}generics: ");
                print_tuple_single(node.generics, tabs + 1);
            }
            println!("{indent}variants: [tuple]");
            print_tuple(node.variants, tabs + 1);
        },
        Node::DataItem(node) => {
            println!("[data item]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.symbol, tabs + 1);
            print!  ("{indent}type: ");
            print_tree_helper(*node.ttype, tabs + 1);
        },
        Node::StructAnon(node) => {
            println!("[struct anon]");
            println!("{indent}fields: [tuple]");
            print_tuple(node.fields, tabs + 1);
        },
        Node::StructDeclaration(node) => {
            println!("[struct declaration]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.symbol, tabs + 1);
            println!("{indent}fields: [tuple]");
            print_tuple(node.fields, tabs + 1);
        }, 
        Node::StructField(node) => {
            println!("[struct field]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.symbol, tabs + 1);
            print!  ("{indent}type: ");
            print_tree_helper(*node.ttype, tabs + 1);
        },
        Node::Package(node) => {
            println!("[package]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::Import(node) => {
            println!("[import]");
            print!  ("{indent}name: ");
            print_tuple_single(node.rhs, tabs + 1);
        },
        Node::MatchExpression(node) => {
            println!("[match]");
            print!  ("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            println!("{indent}branches: [tuple]");
            print_tuple(node.branches, tabs + 1);
        },
        Node::MatchBranch(node) => {
            println!("[match branch]");
            print!  ("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            print!  ("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::ListLiteral(node) => {
            println!("[list literal]");
            print_tuple(node.items, tabs);
        },
        Node::ListSplit(node) => {
            println!("[list split]");
            print!  ("{}head: ", indent);
            print_tree_helper(*node.head, tabs + 1);
            print!  ("{}tail: ", indent);
            print_tree_helper(*node.tail, tabs + 1);  
        },
        Node::TypeFn(node) => {
            println!("[{}]", node.token.typ);
            print!  ("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            print!  ("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);  
        },
        Node::TypeLst(node) => {
            println!("[list type]");
            print!  ("{indent}type: ");
            print_tree_helper(*node.ttype, tabs + 1);
        },
        Node::TypeTuple(node) => {
            println!("[tuple type]");
            print_tuple(node.ttypes, tabs);
        },
        Node::TypeCmpst(node) => {
            println!("[composite type]");
            print!  ("{indent}name: ");
            print_tree_helper(*node.ttype, tabs + 1);
            print!  ("{}items: ", indent);
            print_tuple_single(node.items, tabs + 1);
        },
        Node::Ttype(node) => {
            if node.is_mut {
                print!("mut ");
            }
            print_tree_helper(*node.symbol, tabs)
        },
        Node::WhileExpression(node) => {
            println!("[while]");
            print!  ("{indent}condition: ");
            print_tree_helper(*node.cond, tabs + 1);
            println!("{indent}stmts: [block]");
            print_tuple(node.statements, tabs + 1);
        },
        Node::DoExpression(node) => {
            println!("[do]");
            println!("{indent}stmts: [block]");
            print_tuple(node.statements, tabs + 1);
        },
        Node::ReturnExpression(node) => {
            println!("[return]");
            print!  ("{indent}value: ");
            print_tree_helper(*node.value, tabs + 1);
        },
        Node::BinaryExpression(node) => {
            println!("[{}]", node.token.typ);
            print!  ("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            print!  ("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::UnaryExpression(node) => {
            println!("[{}]", node.token.typ);
            print!  ("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::Call(node) => {
            println!("[call]");
            print!  ("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            if node.args.len() > 0 {
                print!  ("{}arg: ", indent);
                print_tuple_single(node.args, tabs + 1);
            }
        },
        Node::Access(node) => {
            println!("[access]");
            print!("{indent}lhs: ");
            print_tree_helper(*node.lhs, tabs + 1);
            print!("{indent}rhs: ");
            print_tree_helper(*node.rhs, tabs + 1);
        },
        Node::AccessIndex(node) => {
            println!("[access]");
            print!("{indent}lhs: ");
            print_tree_helper(*node.symbol, tabs + 1);
            print!("{indent}rhs: ");
            print_tree_helper(*node.index, tabs + 1);
        },
        Node::TupleAny(node) => {
            println!("[tuple]");
            if node.items.len() > 0 {
                print_tuple(node.items, tabs);
            }
        }
        Node::Atom(node) => {
            let tok = match node.token.typ {
                TokenType::Symbol(atom)  => format!("Symbol '{}'", atom),
                TokenType::Integer(atom) => format!("Int '{}'", atom),
                TokenType::String(atom)  => format!("String \"{}\"", atom),
                _ => format!(""),
            };
            println!("[{}]", tok);
        },
        Node::FnArgTyped(node) => {
            match *node.ttype {
                None => { print_tree_helper(*node.symbol, tabs); },
                Some(ttype) => {
                    println!("[fn argument]");
                    print!  ("{indent}symbol: ");
                    print_tree_helper(*node.symbol, tabs + 1);
                    print!  ("{indent}type: ");
                    print_tree_helper(ttype, tabs + 1);
                },
            }
        },
        _ => println!("<=>")
    }
}

fn print_tuple(nodes: Vec<Node>, tabs: usize) {
    let indent = TAB.repeat(tabs);
    let mut idx = 0;
    for branch in nodes {
        print!("{indent}{idx}: ");
        idx += 1;
        print_tree_helper(branch, tabs + 1);
    }
}

fn print_tuple_single(nodes: Vec<Node>, tabs: usize) {
    let indent = TAB.repeat(tabs);
    if nodes.len() == 0 {
        println!("()");
    }
    if nodes.len() > 1 {
        let mut idx = 0;
        println!("[tuple]");
        for arg in nodes {
            print!("{indent}{idx}: ");
            idx += 1;
            print_tree_helper(arg, tabs + 1);
        }
    } else {
        for arg in nodes {
            print_tree_helper(arg, tabs + 1);
        }
    }
}
