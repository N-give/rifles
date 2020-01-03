use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use termion::{cursor, clear, color};

pub async fn print_dir_contents(pos: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(format!("{}{}", clear::All, cursor::Goto(1, 1)).as_bytes()).await?;

    let mut entries = fs::read_dir(".").await?.enumerate();
    while let Some((i, entry_result)) = entries.next().await {
        stdout.write_all(format_output(entry_result?, pos == i).await?.as_bytes()).await?;
    }
    Ok(())
}

async fn format_output(entry: fs::DirEntry, selected: bool) -> io::Result<String> {
    let name = entry.file_name().into_string().unwrap();
    let mut output = String::new();
    if selected {
        output.push_str(format!("{}", color::Bg(color::LightMagenta)).as_str());
    } /* else {
    } */

    let ft = entry.file_type().await?;
    if ft.is_dir() {
        output.push_str(format!("{}", color::Fg(color::Cyan)).as_str());
    } else if ft.is_file() {
        output.push_str(format!("{}", color::Fg(color::White)).as_str());
    } else {
        output.push_str(format!("{}", color::Fg(color::Yellow)).as_str());
    };

    output.push_str(format!("{}{}{}\r\n", name, color::Fg(color::Reset), color::Bg(color::Reset)).as_str());
    Ok(output)
}
