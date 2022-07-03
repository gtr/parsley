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
    Plus,
    Minus,
    Multiply,
    Divide,
    Root,
}

impl fmt::Display for GrammarItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            GrammarItem::Integer(i) => write!(f, "Integer: {}", i),
            GrammarItem::Plus => write!(f, "+"),
            GrammarItem::Minus => write!(f, "-"),
            GrammarItem::Multiply => write!(f, "*"),
            GrammarItem::Divide => write!(f, "/"),
            GrammarItem::Root => write!(f, "."),
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








