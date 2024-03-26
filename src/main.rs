#![feature(try_trait_v2)]

use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Write;
use std::io::{self, stdout};

mod renderer;
use renderer::render_md;

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut current = String::new();

    enable_raw_mode()?; // Enable raw mode to capture input without buffering
    execute!(stdout, EnterAlternateScreen);

    println!("welcome");
    print!("\x1b[2K\r λ ");
    stdout.flush();

    loop {
        if poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = read()? {
                let CTRL = KeyModifiers::CONTROL;
                match code {
                    KeyCode::Char('d') if modifiers == CTRL => break,
                    KeyCode::Char('c') if modifiers == CTRL => break,

                    KeyCode::Backspace => { 
                        current.pop();
                        render_md(&current);
                        stdout.flush();
                    },
                    KeyCode::Char(c) => {
                        current.push(c);
                        render_md(&current);
                        stdout.flush();
                    }
                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?; // Disable raw mode before exiting
    execute!(stdout, LeaveAlternateScreen);

    Ok(())
}
