#![feature(try_trait_v2)]

use markdown::mdast::{InlineMath, Math};
use std::ops::{ControlFlow, FromResidual, Try};
use unlatex::ast::Node;
// https://oeis.org/wiki/List_of_LaTeX_mathematical_symbols

pub fn render_latex(input: Math) -> String {
    let node = unlatex::parse(&input.value).unwrap();
    //println!("{node:?}");
    render_node(node)
}

pub fn render_latex_inline(input: InlineMath) -> String {
    let node = unlatex::parse(&input.value).unwrap();
    //println!("{node:?}");
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
        Node::Argument { content, .. } => {
            rendered.push_str(&render_nodes(content))
        }

        _ => {}
    }
    rendered
}

pub fn render_macro(content: String, args: Vec<Node>) -> Symbol {
    let rendered_args: Vec<String> = args.iter().map(|arg| render_node(arg.clone())).collect();

    check_greek(&content)?;
    check_ord(&content)?;
    check_bin(&content)?;

    Symbol::Unknown(content, rendered_args)
}

pub fn check_ord(ord: &str) -> Symbol {
    Symbol::Some(
        match ord {
            "ned" => "¬",
            "mp" => "",
            _ => return Symbol::None,
        }
        .to_string(),
    )
}

pub fn check_bin(bin: &str) -> Symbol {
    Symbol::Some(
        match bin {
            "pm" => "±",
            "mp" => "",
            _ => return Symbol::None,
        }
        .to_string(),
    )
}

pub fn check_greek(letter: &str) -> Symbol {
    let upper = letter.chars().next().unwrap_or(' ').is_uppercase();
    let mut symbol = match letter.to_lowercase().as_str() {
        "alpha" => "α",
        "beta" => "β",
        "gamma" => "γ",
        "delta" => "δ",
        "epsilon" => "ε",
        "zeta" => "ζ",
        "eta" => "η",
        "theta" => "θ",
        "iota" => "ι",
        "kappa" => "κ",
        "lambda" => "λ",
        "mu" => "μ",
        "nu" => "ν",
        "xi" => "ξ",
        "omicron" => "ο",
        "pi" => "π",
        "rho" => "ρ",
        "sigma" => "σ",
        "tau" => "τ",
        "upsilon" => "υ",
        "phi" => "φ",
        "chi" => "χ",
        "psi" => "ψ",
        "omega" => "ω",
        _ => return Symbol::None,
    }
    .to_string();
    if upper {
        symbol = symbol.to_uppercase();
    }
    Symbol::Some(symbol)
}

pub enum Symbol {
    Some(String),
    Unknown(String, Vec<String>),
    None,
}

impl Symbol {
    pub fn as_str(self) -> String {
        match self {
            Symbol::Some(s) => s,
            Symbol::Unknown(s, args) => format!("\x1b[31m\\{s}[{args:?}]\x1b[m"),
            Symbol::None => panic!("Symbol is none."),
        }
    }
}

impl FromResidual<String> for Symbol {
    fn from_residual(s: String) -> Self {
        Symbol::Some(s)
    }
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
            Symbol::Unknown(sym, args) => ControlFlow::Break(sym),
            Symbol::None => ControlFlow::Continue(()),
        }
    }
}
