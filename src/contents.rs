use std::{
    ffi::OsString,
    fs,
    io::{self, Error, Write},
};
use termion::{clear, color, cursor};

pub enum EntryType {
    FILE,
    DIRECTORY,
    SYMLINK,
}

pub struct Entry {
    path: String,
    entry_type: EntryType,
}

impl Entry {
    pub fn format(&self, selected: bool) -> String {
        let mut output = String::new();
        if selected {
            output.push_str(&color::Bg(color::LightMagenta).to_string());
        }

        match self.entry_type {
            EntryType::FILE => {
                output.push_str(&color::Fg(color::White).to_string());
            }
            EntryType::DIRECTORY => {
                output.push_str(&color::Fg(color::Cyan).to_string());
            }
            EntryType::SYMLINK => {
                output.push_str(&color::Fg(color::Yellow).to_string());
            }
        }

        output.push_str(&self.path);
        output.push_str(&color::Fg(color::Reset).to_string());
        output.push_str(&color::Bg(color::Reset).to_string());
        output.push_str("\r\n");
        output
    }
}

pub struct Directory {
    working_directory: String,
    position: usize,
    entries: Vec<Entry>,
}

impl Directory {
    pub fn default() -> io::Result<Self> {
        Ok(Directory {
            working_directory: std::env::current_dir()?
                .into_os_string()
                .into_string()
                .map_err(os_string_err)?,
            position: 0,
            entries: fs::read_dir(".")?
                .map(|dr| -> Result<Entry, Error> {
                    let d = dr?;
                    let path = d.file_name().into_string().map_err(os_string_err)?;
                    let file_type = d.file_type()?;
                    let entry_type = if file_type.is_file() {
                        EntryType::FILE
                    } else if file_type.is_dir() {
                        EntryType::DIRECTORY
                    } else {
                        EntryType::SYMLINK
                    };
                    Ok(Entry { path, entry_type })
                })
                .collect::<Result<Vec<Entry>, io::Error>>()?,
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn current_working_directory(&self) -> &str {
        &self.working_directory
    }

    pub fn current_selection(&self) -> &str {
        &self.entries[self.position as usize].path
    }

    pub fn increase_position(&mut self) {
        self.position = self.position.checked_add(1).unwrap_or(self.len() - 1);
    }

    pub fn decrease_position(&mut self) {
        self.position = self.position.checked_sub(1).unwrap_or(0);
    }

    pub fn display_entries(&mut self) -> io::Result<()> {
        let mut display_str = format!("{}{}", clear::All, cursor::Goto(1, 1));
        for (i, entry) in self.entries.iter_mut().enumerate() {
            display_str.push_str(entry.format(self.position as usize == i).as_str());
            if self.position as usize == i {
                self.working_directory = entry.path.clone();
            }
        }
        io::stdout().write_all(display_str.as_bytes())?;
        Ok(())
    }
}

fn os_string_err(os_str: OsString) -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        format!("Failed to convert OsString: {os_str:?}"),
    )
}
