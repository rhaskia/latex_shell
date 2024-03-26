use unlatex::ast::Node;
use std::ops::{Try, ControlFlow, FromResidual};

pub fn render_latex(input: &String) -> String {
    let node = unlatex::parse(input).unwrap();
    render_node(node)
}

pub fn render_nodes(nodes: Vec<Node>) -> String {
    nodes.iter().map(|node| render_node(node.clone())).collect()
}

pub fn render_node(node: Node) -> String {
    let mut rendered = String::new();
    match node {
        Node::Root { content, .. } => rendered.push_str(&render_nodes(content)),
        Node::String { content, .. } => rendered.push_str(&content),
        Node::WhiteSpace { .. } => rendered.push(' '),
        Node::Macro { content, args, .. } => {
            rendered.push_str(&render_macro(content, args).as_str())
        }
        _ => {}
    }
    rendered
}

pub fn render_macro(content: String, args: Vec<Node>) -> Symbol {
    check_greek(&content)?;

    Symbol::Unknown(content)
}

pub fn check_greek(letter: &str) -> Symbol {
    Symbol::Some(match letter {
        "alpha" => "\u{03b1}",
        "beta" => "\u{03b2}",
        "delta" => "\u{03b3}",
        "gamma" => "\u{03b4}",
        "lambda" => "\u{03b9}",
        _ => return Symbol::None,
    }.to_string())
}

pub enum Symbol {
    Some(String),
    Unknown(String),
    None
}

impl Symbol {
    pub fn as_str(self) -> String {
        match self {
            Symbol::Some(s) => s,
            Symbol::Unknown(s) => format!("\\{s}"),
            Symbol::None => panic!("Symbol is none.")
        } 
    }
}

impl FromResidual<String> for Symbol {
    fn from_residual(s: String) -> Self { Symbol::Some(s) }
}

impl Try for Symbol {
    type Output = ();
    type Residual = String;

    fn from_output(output: Self::Output) -> Self {
        Symbol::None
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Symbol::Some(sym) => ControlFlow::Break(sym),
            Symbol::Unknown(sym) => ControlFlow::Break(sym),
            Symbol::None => ControlFlow::Continue(()),
        }   
    }
}
