use std::{io, path::Path};

use termion::event::Key;

use crate::contents::Directory;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Browse,
    Create(Vec<char>),
    Rename(Vec<char>),
    Delete,
    Quit,
}

pub struct State {
    mode: Mode,
    dir: Directory,
}

impl State {
    pub fn default() -> io::Result<Self> {
        Ok(Self {
            mode: Mode::Browse,
            dir: Directory::default()?,
        })
    }

    pub fn display(&mut self) -> io::Result<()> {
        self.dir.display_entries()
    }

    pub fn should_quit(&self) -> bool {
        self.mode == Mode::Quit
    }

    pub fn handle_key(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::Browse => self.handle_key_browse(key),
            Mode::Create(_) => self.handle_key_create(key),
            Mode::Rename(_) => self.handle_key_rename(key),
            Mode::Delete => self.handle_key_delete(key),
            Mode::Quit => unreachable!(
                "Mode::handle_key should never be called after Mode::Quit has been returned"
            ),
        }
    }

    pub fn handle_key_browse(&mut self, key: Key) -> io::Result<()> {
        match key {
            Key::Char('q') => self.mode = Mode::Quit,

            // Navigate within a directory
            Key::Char('j') => self.dir.increase_position(),
            Key::Char('k') => self.dir.decrease_position(),

            // Move up a directory
            Key::Char('h') => {
                std::env::set_current_dir(Path::new(".."))?;
                self.dir = Directory::default()?;
            }
            // Navigate into the current directory
            // TODO handle current path isn't a directory
            Key::Char('l') => {
                std::env::set_current_dir(Path::new(self.dir.current_selection()))?;
                self.dir = Directory::default()?;
            }

            Key::Char('m') => {
                self.mode = Mode::Rename(self.dir.current_selection().chars().collect())
            }
            Key::Char('n') => self.mode = Mode::Create(Vec::new()),
            Key::Char('d') => self.mode = Mode::Delete,
            _ => {
                todo!("Invalid Character: {key:?}");
            }
        }
        Ok(())
    }

    fn handle_key_create(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::Create(ref mut to_create) => match key {
                Key::Char('\n') => todo!("create the new item {:?}", to_create),
                Key::Char(c) => to_create.push(c),
                Key::Backspace => {
                    to_create.pop();
                }
                _ => {
                    todo!("unhandled key in `Mode::handle_key_create` {key:?}");
                }
            },
            _ => unreachable!(
                "`Mode::handle_key_create` should only be called while mode is `Mode::Create`"
            ),
        };
        Ok(())
    }

    fn handle_key_rename(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::Rename(ref mut to_create) => match key {
                Key::Char(c) => to_create.push(c),
                Key::Backspace => {
                    to_create.pop();
                }
                _ => {
                    todo!("unhandled key in `Mode::handle_key_rename` {key:?}");
                }
            },
            _ => unreachable!(
                "`Mode::handle_key_rename` should only be called while mode is `Mode::Rename`"
            ),
        }
        Ok(())
    }

    fn handle_key_delete(&mut self, key: Key) -> io::Result<()> {
        todo!();
    }
}
