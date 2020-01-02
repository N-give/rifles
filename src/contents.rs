use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use termion::{cursor, clear, color};

pub async fn print_dir_contents(_pos: usize) -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("{}{}", clear::All, cursor::Goto(1, 1)).as_bytes()).await?;

    let mut entries = fs::read_dir(".").await?;
    while let Some(entry_result) = entries.next().await {
        let entry = entry_result?;
        let mut output = String::new();

        let ft = entry.file_type().await?;
        if ft.is_dir() {
            output.push_str(format!("{}", color::Fg(color::Cyan)).as_str());
        } else if ft.is_file() {
            output.push_str(format!("{}", color::Fg(color::White)).as_str());
        } else {
            output.push_str(format!("{}", color::Fg(color::Yellow)).as_str());
        };

        output.push_str(format!(
                "{}{}\r\n",
                entry.file_name().into_string().unwrap(),
                color::Fg(color::Reset)).as_str());

        stdout.write_all(output.as_bytes()).await?;
    }
    Ok(())
}
