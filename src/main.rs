use std::{
    io::{self, stdin, stdout, Write},
    path::Path,
};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

use crate::contents::Directory;

mod contents;

fn main() -> Result<(), io::Error> {
    let stdin = stdin();
    let _raw_stdout = stdout().into_raw_mode().expect("Failed to enter raw mode");
    let mut stdout = io::stdout();

    stdout.write_all(format!("{}", cursor::Hide).as_bytes())?;

    let mut current_dir = Directory::default()?;
    current_dir.display_entries()?;

    for key in stdin.keys() {
        match key.unwrap() {
            // Quit and close rifles
            Key::Char('q') => break,

            // Navigate within a directory
            Key::Char('j') => current_dir.increase_position(),
            Key::Char('k') => current_dir.decrease_position(),

            // Move up a directory
            Key::Char('h') => {
                std::env::set_current_dir(Path::new(".."))?;
                current_dir = Directory::default()?;
            }
            // Navigate into the current directory
            // TODO handle current path isn't a directory
            Key::Char('l') => {
                std::env::set_current_dir(Path::new(current_dir.current_selection()))?;
                current_dir = Directory::default()?;
            }

            Key::Char('n') => {
                todo!("handle creating a new path");
            }
            Key::Char('d') => {
                todo!("handle deleting the current path");
            }
            _ => eprintln!("Invalid Character"),
        }

        current_dir.display_entries()?;
    }

    stdout
        .write_all(format!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).as_bytes())
        .unwrap();

    Ok(())
}
