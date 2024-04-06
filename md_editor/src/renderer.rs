use crate::editor::Cursor;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use fehler::throws;
use latex_renderer::{render_latex, render_latex_inline};
use markdown::mdast::{Heading, List, ListItem, Node, Paragraph, Table, ThematicBreak};
use markdown::unist::Position;
use markdown::*;
use std::io::Error;
use std::io::Write;

pub struct Drawer {
    out: std::io::Stdout,
    screen: Vec<Line>,
    md_opt: ParseOptions,
    max_width: usize,
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

const DOUBLE_TOP: &str = "\x1b#3";
const DOUBLE_BOTTOM: &str = "\x1b#4";

const EM: &str = "\x1b[3m";
const END_EM: &str = "\x1b[23m";

const STRONG: &str = "\x1b[1m";
const END_STRONG: &str = "\x1b[22m";

const GREY: &str = "\x1b[90m";
const WHITE: &str = "\x1b[37m";

impl Drawer {
    pub fn new() -> Self {
        let mut md_opt = ParseOptions::gfm();
        md_opt.constructs.math_text = true;
        Drawer { out: std::io::stdout(), md_opt, screen: Vec::new(), max_width: 10 }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.max_width = rows;
    }

    #[throws]
    pub fn render_md(&mut self, file: Vec<String>, cursor: Cursor) {
        self.screen = Vec::new();
        let tree = to_mdast(&file.join("\n"), &self.md_opt).unwrap();

        execute!(&self.out, Clear(ClearType::All), MoveTo(0, 0))?;

        self.render_node(tree.clone());
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

        print!("\n\r{:?}", tree);
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
            List(list) => self.render_list(list),
            Table(table) => self.render_table(table),
            ThematicBreak(br) => self.render_break(br),
            _ => println!("{node:?}"),
        };
    }

    pub fn render_break(&mut self, br: ThematicBreak) {
        let Position { start, end, .. } = br.position.unwrap();
        self.ensure_scr_lines(end.line);

        self.screen[start.line - 1] = Line::from(format!(" {:─^1$} ", "", self.max_width - 2));
    }

    pub fn render_table(&mut self, table: Table) {
        let Position { start, end, .. } = table.position.unwrap();
        self.ensure_scr_lines(end.line);

        let mut rows = Vec::new();
        for (row_idx, row) in table.children.iter().enumerate() {
            let row =
                if let Node::TableRow(tr) = row { tr } else { panic!("Non-TableRow in table") };
            rows.push((Vec::new(), row.position.clone().unwrap().start.line));

            for cell in row.children.iter() {
                let cell = if let Node::TableCell(tc) = cell {
                    tc
                } else {
                    panic!("Non-TableCell in table")
                };
                let children = self.render_children(cell.children.clone());
                rows[row_idx].0.push(children);
            }
        }

        let mut col_widths: Vec<usize> = vec![0; rows[0].0.len()];
        for row in rows.iter() {
            for (i, cell) in row.0.iter().enumerate() {
                col_widths[i] = (*col_widths.get(i).unwrap_or(&0)).max(cell.len());
            }
        }

        // seperator line
        self.screen[start.line] = Line::from(self.table_sep(col_widths.clone()));

        // actual table rendering
        for (row, row_idx) in rows {
            self.screen[row_idx - 1] = Line::from(self.render_table_row(row, col_widths.clone()));
        }
    }

    pub fn table_sep(&self, widths: Vec<usize>) -> String {
        let parts: Vec<String> = widths.iter().map(|w| format!("{:─^1$}", "", w + 2)).collect();
        format!("├{}┤", parts.join("┼"))
    }

    pub fn render_table_row(&mut self, row: Vec<String>, widths: Vec<usize>) -> String {
        let spaced =
            row.iter().zip(widths).map(|(r, w)| format!("{:^1$}", r, w)).collect::<Vec<String>>();
        let conjoined = spaced.join(" │ ");
        format!("│ {conjoined} │")
    }

    pub fn render_list(&mut self, list: List) {
        let dot = format!("{GREY}\u{f444}{WHITE}");
        for (idx, child) in list.children.iter().enumerate() {
            let li = if let Node::ListItem(li) = child {
                li
            } else {
                panic!("Non-ListItem in List");
            };

            let marker = if list.ordered { format!("{}.", idx + 1) } else { dot.clone() };
            self.render_list_item(li.clone(), &marker);
        }
    }

    pub fn render_list_item(&mut self, item: ListItem, marker: &str) {
        let Position { start, end, .. } = item.position.unwrap();
        self.ensure_scr_lines(end.line);
        let children = format!("{marker} {}", self.render_children(item.children));

        let lines = children.lines().enumerate();
        for (idx, line) in lines {
            self.screen[start.line + idx - 1] = Line::from(line.to_string());
        }
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
            Emphasis(text) => format!("{EM}{}{END_EM}", self.render_children(text.children)),
            Strong(text) => format!("{STRONG}{}{END_STRONG}", self.render_children(text.children)),

            BlockQuote(_) => todo!(),
            FootnoteDefinition(_) => todo!(),
            Toml(_) => todo!(),
            Yaml(_) => todo!(),
            Break(_) => todo!(),
            InlineCode(_) => todo!(),
            InlineMath(math) => render_latex_inline(math),
            Delete(del) => format!("\x1b[9m]{}\x1b[29m", self.render_children(del.children)),
            FootnoteReference(_) => todo!(),
            Html(_) => todo!(),
            Image(_) => todo!(),
            ImageReference(_) => todo!(),
            Link(_) => todo!(),
            LinkReference(_) => todo!(),
            Code(_) => todo!(),
            Math(math) => render_latex(math),
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
        self.screen[start.line - 1] =
            Line::double(format!("{DOUBLE_TOP}{inner}\r\n{DOUBLE_BOTTOM}{inner}"));
    }
}
