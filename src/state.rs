use crate::contents::Directory;
use std::{
    fs::{create_dir, rename, File},
    io::{self, Write},
    path::Path,
};
use termion::{clear, cursor, event::Key, terminal_size};

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Browse,
    New,
    CreateFile(Vec<char>),
    CreateDir(Vec<char>),
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
        self.dir.display_entries()?;
        match &self.mode {
            Mode::Browse => Ok(()),
            Mode::New => display_type_selection(),
            Mode::CreateFile(_) => self.display_mutation(),
            Mode::CreateDir(_) => self.display_mutation(),
            Mode::Rename(_) => self.display_mutation(),
            Mode::Delete => todo!("State::display => handle delete"),
            Mode::Quit => Ok(()),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.mode == Mode::Quit
    }

    pub fn handle_key(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::Browse => self.handle_key_browse(key),
            Mode::New => self.handle_key_new(key),
            Mode::CreateFile(_) => self.handle_key_create_file(key),
            Mode::CreateDir(_) => self.handle_key_create_dir(key),
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
            Key::Char('l') | Key::Char('\n') => match self.dir.current_selection().entry_type {
                crate::contents::EntryType::FILE => todo!("handle current path isn't a directory"),
                crate::contents::EntryType::DIRECTORY | crate::contents::EntryType::SYMLINK => {
                    std::env::set_current_dir(Path::new(&self.dir.current_selection().path))?;
                    self.dir = Directory::default()?;
                }
            },

            Key::Char('m') => {
                self.mode = Mode::Rename(self.dir.current_selection().path.chars().collect())
            }
            Key::Char('n') => self.mode = Mode::New,
            Key::Char('d') => self.mode = Mode::Delete,
            _ => {
                todo!("Invalid Character: {key:?}");
            }
        }
        Ok(())
    }

    fn handle_key_new(&mut self, key: Key) -> io::Result<()> {
        match key {
            Key::Char('f') => self.mode = Mode::CreateFile(Vec::new()),
            Key::Char('d') => self.mode = Mode::CreateDir(Vec::new()),
            Key::Esc => self.mode = Mode::Browse,
            _ => todo!("State::handle_key_new: unknown key ({key:?})"),
        }
        Ok(())
    }

    fn handle_key_create_file(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::CreateFile(ref mut to_create) => match key {
                Key::Char('\n') => {
                    File::create_new(to_create.iter().collect::<String>())?;
                    self.dir = Directory::default()?;
                    self.mode = Mode::Browse;
                }
                Key::Char(c) => to_create.push(c),
                Key::Backspace => {
                    to_create.pop();
                }
                Key::Esc => self.mode = Mode::Browse,
                _ => {
                    todo!("unhandled key in `Mode::handle_key_create_file` {key:?}");
                }
            },
            _ => unreachable!(
                "`Mode::handle_key_create_file` should only be called while mode is `Mode::CreateFile`"
            ),
        };
        Ok(())
    }

    fn handle_key_create_dir(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::CreateDir(ref mut to_create) => match key {
                Key::Char('\n') => {
                    create_dir(to_create.iter().collect::<String>())?;
                    self.dir = Directory::default()?;
                    self.mode = Mode::Browse;
                }
                Key::Char(c) => to_create.push(c),
                Key::Backspace => {
                    to_create.pop();
                }
                Key::Esc => self.mode = Mode::Browse,
                _ => {
                    todo!("unhandled key in `Mode::handle_key_create_dir` {key:?}");
                }
            },
            _ => unreachable!(
                "`Mode::handle_key_create_dir` should only be called while mode is `Mode::CreateDir`"
            ),
        };
        Ok(())
    }

    fn handle_key_rename(&mut self, key: Key) -> io::Result<()> {
        match self.mode {
            Mode::Rename(ref mut to_rename) => match key {
                Key::Char('\n') => {
                    rename(
                        &Path::new(&self.dir.current_selection().path),
                        &Path::new(&to_rename.iter().collect::<String>()),
                    )?;
                    self.dir = Directory::default()?;
                    self.mode = Mode::Browse;
                }
                Key::Char(c) => to_rename.push(c),
                Key::Backspace => {
                    to_rename.pop();
                }
                Key::Esc => self.mode = Mode::Browse,
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

    fn handle_key_delete(&mut self, _key: Key) -> io::Result<()> {
        todo!();
    }

    fn display_mutation(&self) -> io::Result<()> {
        let (_cols, rows) = terminal_size()?;
        let prompt = match &self.mode {
            Mode::CreateFile(new_name) => {
                format!("Filename: {}", new_name.iter().collect::<String>())
            }
            Mode::CreateDir(new_name) => {
                format!("Directory: {}", new_name.iter().collect::<String>())
            }
            Mode::Rename(new_name) => {
                let entry_type = match self.dir.current_selection().entry_type {
                    crate::contents::EntryType::FILE => "Filename",
                    crate::contents::EntryType::DIRECTORY => "Directory",
                    crate::contents::EntryType::SYMLINK => "Symlink",
                };
                format!("{entry_type}: {}", new_name.iter().collect::<String>())
            }
            _ => unreachable!(),
        };
        let output = format!("{}{}{}", cursor::Goto(1, rows), clear::CurrentLine, prompt,);
        let mut stdio = io::stdout();
        stdio.write_all(output.as_bytes())?;
        stdio.flush()?;

        Ok(())
    }
}

fn display_type_selection() -> io::Result<()> {
    let (_cols, rows) = terminal_size()?;
    let mut output = format!(
        "{}{}New File(f)",
        cursor::Goto(1, rows - 1),
        clear::CurrentLine,
    );
    output.push_str(&format!(
        "{}{}New Directory(d)",
        cursor::Goto(1, rows),
        clear::CurrentLine,
    ));
    let mut stdio = io::stdout();
    stdio.write_all(output.as_bytes())?;
    stdio.flush()?;
    Ok(())
}
