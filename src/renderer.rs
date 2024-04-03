use markdown::*;
use markdown::mdast::{Node, Heading, Text, Paragraph};
use markdown::unist::Position;
use std::io::Write;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, 
    disable_raw_mode, enable_raw_mode,
    EnterAlternateScreen, LeaveAlternateScreen
};
use crossterm::cursor::MoveTo;

pub struct Drawer {
    out: std::io::Stdout,    
    screen: Vec<String>,
}

impl Drawer {
    pub fn render_md(&mut self, file: &String) {
        self.screen = Vec::new();
        let mut options = ParseOptions::default();
        options.constructs.math_text = true;
        let tree = match to_mdast(file, &options) {
            Ok(t) => t,
            Err(err) => {
                eprintln!("Markdown failed to render @{err:?}");
                return;
            }
        };

        execute!(&self.out, Clear(ClearType::All), MoveTo(0, 0));

        self.render_node(tree);
        print!("{}", self.screen.join("\r\n"));
        self.out.flush();
    }

    pub fn alt_screen(&mut self, active: bool) {
        if active {
            enable_raw_mode().unwrap(); // Enable raw mode to capture input without buffering
            execute!(&self.out, EnterAlternateScreen);
        } else {
            disable_raw_mode();
            execute!(&self.out, LeaveAlternateScreen);
        }
    } 

    pub fn render_nodes(&mut self, nodes: Vec<Node>) {
        for node in nodes { self.render_node(node); }
    }

    pub fn ensure_lines(&mut self, lines: usize) { 
        if self.screen.len() >= lines { return }
        self.screen.extend(vec![String::new(); lines - self.screen.len()]);
    }

    pub fn render_node(&mut self, node: Node) {
        use mdast::Node::*;
        match node {
            Root(root) => self.render_nodes(root.children),
            Paragraph(para) => self.render_para(para),
//            Text(content) => self.render_text(content),
            Heading(head) => self.render_header(head),
            _ => println!("{node:?}")
        };
    }

    pub fn render_para(&mut self, para: Paragraph) {
        let Position { start, end, .. } = para.position.unwrap();
        self.ensure_lines(end.line);
        let children = self.render_children(para.children);
        for (idx, line) in children.lines().enumerate() {
            self.screen[start.line + idx - 1] = line.to_string();
        }
    }

    pub fn render_children(&mut self, nodes: Vec<Node>) -> String { 
        nodes.iter().map(|node| self.render_child(node.clone())).collect()
    }

    pub fn render_child(&mut self, child: Node) -> String {
        use mdast::Node::*;
        match child {
            Emphasis(text) => format!("\x1b[4m{}", self.render_children(text.children)),
            Text(text) => text.value,
            _ => String::new()
        }
    }

    pub fn render_header(&mut self, header: Heading) {
        let Position { start, end, .. } = header.position.unwrap();
        self.ensure_lines(end.line);
        let inner = self.render_children(header.children);
        //let inner = self.render_nodes(header.children);
        self.screen[start.line - 1] = format!("\x1b#3{inner}\r\n\x1b#4{inner}");
    }

    pub fn new() -> Self {
        Drawer { 
            out: std::io::stdout(),
            screen: Vec::new()
        }
    }
}
