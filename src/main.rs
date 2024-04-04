#![feature(try_trait_v2)]

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::event::{KeyEventKind, KeyModifiers};
use std::io;

mod renderer;
use renderer::Drawer;
mod editor;
use editor::Editor;
use markdown::to_mdast;
use markdown::ParseOptions;

fn main() -> io::Result<()> {
    let mut drawer = Drawer::new();
    let mut editor = Editor::new();

    drawer.alt_screen(true)?;

    loop {
        if poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = read()?
            {
                let ctrl = KeyModifiers::CONTROL;
                match code {
                    KeyCode::Char('d') if modifiers == ctrl => break,
                    KeyCode::Char('c') if modifiers == ctrl => break,

                    KeyCode::Up => editor.cursor_up(),
                    KeyCode::Down => editor.cursor_down(),
                    KeyCode::Left => editor.cursor_left(),
                    KeyCode::Right => editor.cursor_right(),

                    KeyCode::Enter => editor.new_line(),
                    KeyCode::Backspace => editor.backspace(),
                    KeyCode::Char(c) => editor.push(c),

                    _ => (),
                }

                drawer.render_md(editor.get_file(), editor.get_cursor())?;
            }
        }
    }

    drawer.alt_screen(false)?;

    Ok(())
}
