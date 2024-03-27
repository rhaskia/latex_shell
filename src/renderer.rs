use markdown::*;
use markdown::mdast::Node;
use std::io::Write;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, 
    disable_raw_mode, enable_raw_mode,
    EnterAlternateScreen, LeaveAlternateScreen
};
use crossterm::cursor::MoveTo;

pub struct Drawer {
    out: std::io::Stdout,    
}

impl Drawer {
    pub fn render_md(&mut self, file: &String) {
        let options = ParseOptions::default();
        let tree = match to_mdast(file, &options) {
            Ok(t) => t,
            Err(err) => {
                eprintln!("Markdown failed to render @{err:?}");
                return;
            }
        };

        execute!(&self.out, Clear(ClearType::All), MoveTo(0, 0));

        self.render_node(tree);

        self.out.flush();
    }

    pub fn alt_screen(&mut self, active: bool) {
        if active {
            enable_raw_mode().unwrap(); // Enable raw mode to capture input without buffering
            execute!(&self.out, EnterAlternateScreen);
        } else {
            execute!(&self.out, LeaveAlternateScreen);
            disable_raw_mode();
        }
    } 

    pub fn render_nodes(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.render_node(node);
        }
    }

    pub fn render_node(&mut self, node: Node) {
        use mdast::Node::*;
        match node {
            Root(root) => self.render_nodes(root.children),
            Paragraph(para) => self.render_nodes(para.children),
            _ => println!("{node:?}")
        }
    }

    pub fn new() -> Self {
        Drawer { 
            out: std::io::stdout(),
        }
    }
}
