use std::io::{self, stdin, stdout, Write};
use termion::{clear, cursor, input::TermRead, raw::IntoRawMode};

use crate::state::State;

mod contents;
mod state;

fn main() -> io::Result<()> {
    let stdin = stdin();
    let _raw_stdout = stdout().into_raw_mode()?;
    let mut stdout = io::stdout();

    stdout.write_all(format!("{}", cursor::Hide).as_bytes())?;

    let mut state = State::default()?;
    state.display()?;

    for key in stdin.keys() {
        state.handle_key(key?)?;
        state.display()?;
        if state.should_quit() {
            break;
        }
    }

    stdout
        .write_all(format!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).as_bytes())
        .unwrap();

    Ok(())
}
