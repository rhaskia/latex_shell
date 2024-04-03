#![feature(try_trait_v2)]

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::event::{KeyEventKind, KeyModifiers};
use std::io;

mod renderer;
use renderer::Drawer;

fn main() -> io::Result<()> {
    let mut drawer = Drawer::new();

    drawer.alt_screen(true)?;

    loop {
        if poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = read()?
            {
                let ctrl = KeyModifiers::CONTROL;
                match code {
                    KeyCode::Char('d') if modifiers == ctrl => break,
                    KeyCode::Char('c') if modifiers == ctrl => break,

                    KeyCode::Enter => drawer.new_line(),
                    KeyCode::Backspace => drawer.backspace(),
                    KeyCode::Char(c) => drawer.push(c),

                    _ => (),
                }
                drawer.render_md()?;
            }
        }
    }

    drawer.alt_screen(false)?;

    Ok(())
}
