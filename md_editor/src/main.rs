#![feature(try_trait_v2)]

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::event::{KeyEventKind, KeyModifiers};
use std::io::{self, stdout};

mod renderer;
use renderer::Drawer;
mod editor;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use editor::Editor;

fn main() -> io::Result<()> {
    let mut drawer = Drawer::new();
    let mut editor = Editor::new();

    drawer.alt_screen(true)?;
    let (rows, cols) = crossterm::terminal::size()?;
    drawer.resize(rows.into(), cols.into());

    loop {
        if !poll(std::time::Duration::from_millis(50))? {
            continue;
        }

        match read()? {
            Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) => { 
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
                };
            },
            Event::Paste(to_paste) => editor.paste(to_paste),
            Event::Resize(rows, cols) => { 
                drawer.resize(rows.into(), cols.into());
            },
            _ => {}
        }

        crossterm::execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        drawer.render_md(editor.get_file(), editor.get_cursor())?;
    }

    drawer.alt_screen(false)?;

    Ok(())
}
