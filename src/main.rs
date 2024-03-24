use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::Write;
use std::io::{self, stdout};

mod renderer;
use renderer::render_latex;

fn main() -> io::Result<()> {
    enable_raw_mode()?; // Enable raw mode to capture input without buffering

    let mut stdout = stdout();
    let mut current = String::new();

    println!("Type something (press 'q' to quit):");

    loop {
        if poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = read()?
            {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) if modifiers == KeyModifiers::SHIFT => {}
                    KeyCode::Char(c) => {
                        current.push(c);
                        print!("\r $ {}", render_latex(&current));
                        stdout.flush();
                    }
                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?; // Disable raw mode before exiting

    Ok(())
}
