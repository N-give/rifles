use async_std::task;
use async_std::io;
use std::io::{stdin, stdout};
use async_std::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use termion::clear;

mod contents;

fn main() {
    let stdin = stdin();
    let _raw_stdout = stdout().into_raw_mode().expect("Failed to enter raw mode");
    let mut stdout = io::stdout();

    task::block_on(async move {
        stdout.write_all(format!("{}", cursor::Hide).as_bytes()).await.unwrap();

        let mut current_position = 0;
        let mut current_length = 0;
        contents::print_dir_contents(current_position)
            .await
            .expect("Failed to read directory contents");

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                // Key::Char('h') => unimplemented!(),
                Key::Char('j') => {
                    if current_position < current_length {
                        current_position += 1;
                    }
                }

                Key::Char('k') => {
                    if current_position > 0 {
                        current_position -= 1;
                    }
                }

                // Key::Char('l') => unimplemented!(),
                _ => eprintln!("Invalid Character"),
            }

            current_length = contents::print_dir_contents(current_position)
                .await
                .expect("Failed to write directory contents");
        }

        stdout.write_all(
            format!(
                "{}{}{}",
                clear::All,
                cursor::Goto(1, 1),
                cursor::Show
            ).as_bytes()
        ).await.unwrap();
    });
}
