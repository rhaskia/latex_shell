use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use fehler::throws;
use markdown::mdast::{Heading, ListItem, Node, Paragraph, Text};
use markdown::unist::Position;
use markdown::*;
use std::io::Error;
use std::io::Write;
use crate::editor::Cursor;

pub struct Drawer {
    out: std::io::Stdout,
    screen: Vec<Line>,
    md_opt: ParseOptions,
}

#[derive(Clone)]
pub struct Line {
    pub inner: String,
    pub size: usize,
}

impl Line {
    pub fn new() -> Self {
        Line { inner: String::new(), size: 1 }
    }

    pub fn from(s: String) -> Self {
        Line { inner: s, size: 1 }
    }

    pub fn double(s: String) -> Self {
        Line { inner: s, size: 2 }
    }
}

impl Drawer {
    #[throws]
    pub fn render_md(&mut self, file: Vec<String>, cursor: Cursor) {
        self.screen = Vec::new();
        let tree = to_mdast(&file.join("\n"), &self.md_opt).unwrap();

        execute!(&self.out, Clear(ClearType::All), MoveTo(0, 0))?;

        self.render_node(tree);
        self.ensure_scr_lines(cursor.line + 1);
        let (mut draw_pos, mut cursor_draw) = (0, 0);
        for (idx, line) in self.screen.iter().enumerate() {
            if idx == cursor.line {
                print!("{}\r\n", file[idx]);
                cursor_draw = draw_pos;
                draw_pos += 1;
            } else {
                print!("{}\r\n", line.inner);
                draw_pos += line.size;
            }
        }
        execute!(&self.out, MoveTo(cursor.col as u16, cursor_draw as u16))?;
        self.out.flush()?;
    }

    #[throws]
    pub fn alt_screen(&mut self, active: bool) {
        if active {
            enable_raw_mode().unwrap(); // Enable raw mode to capture input without buffering
            execute!(&self.out, EnterAlternateScreen)?;
        } else {
            disable_raw_mode()?;
            execute!(&self.out, LeaveAlternateScreen)?;
        }
    }

    pub fn render_nodes(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.render_node(node);
        }
    }

    pub fn ensure_scr_lines(&mut self, lines: usize) {
        if self.screen.len() > (lines + 1) {
            return;
        }
        self.screen.extend(vec![Line::new(); (lines + 1) - self.screen.len()]);
    }

    pub fn render_node(&mut self, node: Node) {
        use mdast::Node::*;
        match node {
            Root(root) => self.render_nodes(root.children),
            Paragraph(para) => self.render_para(para),
            Heading(head) => self.render_header(head),
            List(list) => self.render_nodes(list.children),
            ListItem(item) => self.render_list_item(item),
            _ => println!("{node:?}"),
        };
    }

    pub fn render_list_item(&mut self, item: ListItem) {
        let Position { start, end, .. } = item.position.unwrap();
        self.ensure_scr_lines(end.line);
        let children = self.render_children(item.children);
        self.screen[start.line - 1] = Line::from(format!("\u{f444} {}", children));
    }

    pub fn render_para(&mut self, para: Paragraph) {
        let Position { start, end, .. } = para.position.unwrap();
        self.ensure_scr_lines(end.line);
        let children = self.render_children(para.children);
        for (idx, line) in children.lines().enumerate() {
            self.screen[start.line + idx - 1] = Line::from(line.to_string());
        }
    }

    pub fn render_children(&mut self, nodes: Vec<Node>) -> String {
        nodes.iter().map(|node| self.render_child(node.clone())).collect()
    }

    pub fn render_child(&mut self, child: Node) -> String {
        use mdast::Node::*;
        match child {
            Text(text) => text.value,
            Emphasis(text) => format!("\x1b[3m{}\x1b[23m", self.render_children(text.children)),
            Strong(text) => format!("\x1b[1m{}\x1b[22m", self.render_children(text.children)),

            BlockQuote(_) => todo!(),
            FootnoteDefinition(_) => todo!(),
            List(_) => todo!(),
            Toml(_) => todo!(),
            Yaml(_) => todo!(),
            Break(_) => todo!(),
            InlineCode(_) => todo!(),
            InlineMath(_) => todo!(),
            Delete(del) => format!("\x1b[9m]{}\x1b[29m", self.render_children(del.children)),
            FootnoteReference(_) => todo!(),
            Html(_) => todo!(),
            Image(_) => todo!(),
            ImageReference(_) => todo!(),
            Link(_) => todo!(),
            LinkReference(_) => todo!(),
            Code(_) => todo!(),
            Math(_) => todo!(),
            Heading(_) => todo!(),
            Table(_) => todo!(),
            ThematicBreak(_) => todo!(),

            TableRow(_) => todo!(),
            TableCell(_) => todo!(),

            ListItem(_) => todo!(),
            Definition(_) => todo!(),

            MdxJsxTextElement(_) => todo!(),
            MdxTextExpression(_) => todo!(),
            MdxFlowExpression(_) => todo!(),
            MdxJsxFlowElement(_) => todo!(),
            MdxjsEsm(_) => todo!(),

            Paragraph(para) => self.render_children(para.children),
            _ => panic!("Node should not show nested: {:?}", child),
        }
    }

    pub fn render_header(&mut self, header: Heading) {
        let Position { start, end, .. } = header.position.unwrap();
        self.ensure_scr_lines(end.line);
        let inner = self.render_children(header.children);
        self.screen[start.line - 1] = Line::double(format!("\x1b#3{inner}\r\n\x1b#4{inner}"));
    }

    pub fn new() -> Self {
        let mut md_opt = ParseOptions::default();
        md_opt.constructs.math_text = true;
        Drawer {
            out: std::io::stdout(),
            md_opt,
            screen: Vec::new(),
        }
    }
}
