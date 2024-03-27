#![feature(try_trait_v2)]

use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
};
use std::io::Write;
use std::io::{self, stdout};

mod renderer;
use renderer::Drawer;

fn main() -> io::Result<()> {
    let mut current = String::new();
    let mut drawer = Drawer::new();

    drawer.alt_screen(true);

    loop {
        if poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = read()? {
                let CTRL = KeyModifiers::CONTROL;
                match code {
                    KeyCode::Char('d') if modifiers == CTRL => break,
                    KeyCode::Char('c') if modifiers == CTRL => break,

                    KeyCode::Enter => {
                        current.push('\n');
                        drawer.render_md(&current);
                    }

                    KeyCode::Backspace => { 
                        current.pop();
                        drawer.render_md(&current);
                    },
                    KeyCode::Char(c) => {
                        current.push(c);
                        drawer.render_md(&current);
                    }
                    _ => (),
                }
            }
        }
    }

    drawer.alt_screen(false);

    Ok(())
}
