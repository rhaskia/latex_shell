use unlatex::ast::Node;

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
        unlatex::ast::Node::Root { content, .. } => rendered.push_str(&render_nodes(content)),
        unlatex::ast::Node::String { content, .. } => rendered.push_str(&content),
        unlatex::ast::Node::WhiteSpace { .. } => rendered.push(' '),
        unlatex::ast::Node::Macro { content, args, .. } => {
            rendered.push_str(&render_macro(content, args))
        }
        _ => {}
    }
    rendered
}

pub fn render_macro(content: String, args: Vec<Node>) -> String {
    content
}
