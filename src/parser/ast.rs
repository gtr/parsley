use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub entry: GrammarItem,
}

impl Node {
    pub fn new(entry: GrammarItem) -> Node {
        Node {
            children: Vec::new(),
            entry: entry,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GrammarItem {
    Integer(i32),
    Symbol(String),
    True,
    False,
    
    NotEq,
    Tuple,
    
    Plus,
    PlusPlus,
    Minus,
    MinusMimus,
    Multiply,
    Divide,
    
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Not,
    Eq,
    Root,
    And,
    Or,
    
    Let,
    Fn,
    If,
}

impl fmt::Display for GrammarItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            GrammarItem::Integer(i) => write!(f, "Integer: {}", i),
            GrammarItem::Symbol(s) => write!(f, "Symbol: {}", s),
            GrammarItem::True => write!(f, "true"),
            GrammarItem::False => write!(f, "false"),

            GrammarItem::Plus => write!(f, "+"),
            GrammarItem::PlusPlus => write!(f, "++"),
            GrammarItem::Minus => write!(f, "-"),
            GrammarItem::MinusMimus => write!(f, "--"),
            GrammarItem::Multiply => write!(f, "*"),
            GrammarItem::Divide => write!(f, "/"),
            GrammarItem::Greater => write!(f, ">"),
            GrammarItem::GreaterEqual => write!(f, ">="),
            GrammarItem::Less => write!(f, "<"),
            GrammarItem::LessEqual => write!(f, "<="),
            GrammarItem::Eq => write!(f, "=="),
            GrammarItem::NotEq => write!(f, "!="),
            GrammarItem::Not => write!(f, "!"),
            GrammarItem::Root => write!(f, "."),
            GrammarItem::Tuple => write!(f, "tuple"),
            
            GrammarItem::And => write!(f, "and"),
            GrammarItem::Or => write!(f, "or"),
            
            GrammarItem::Let => write!(f, "let"),
            GrammarItem::Fn => write!(f, "fn"),
            GrammarItem::If => write!(f, "if"),
        }
    }
}

pub fn print_tree(root: &Node) {
    print_tree_helper(root, 0);
}

pub fn print_tree_helper(root: &Node, level: usize) {
    match root.entry {
        GrammarItem::Root => {},
        _ => {
            let indent = "  ".repeat(level - 1);
            println!("{}: {}{}", level, indent, root.entry);
        }
    }

    for ch in &root.children {
        print_tree_helper(&ch, level + 1);
    }
}
