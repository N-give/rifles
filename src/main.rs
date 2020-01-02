use async_std::task;
use std::io::{stdin, stdout};
// use async_std::prelude::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod contents;

fn main() {
    let stdin = stdin();
    let _stdout = stdout().into_raw_mode().expect("Failed to enter raw mode");

    task::block_on(async move {
        let mut current_position = 0;
        contents::print_dir_contents(current_position)
            .await
            .expect("Failed to read directory contents");

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                // Key::Char('h') => unimplemented!(),
                Key::Char('j') => current_position += 1,
                Key::Char('k') => current_position -= 1,
                // Key::Char('l') => unimplemented!(),
                _ => eprintln!("Invalid Character"),
            }
            contents::print_dir_contents(current_position)
                .await
                .expect("Failed to write directory contents");
        }
    });
}
