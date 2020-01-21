use async_std::{fs, io, prelude::*};
use termion::{cursor, clear, color};

pub async fn print_dir_contents(pos: usize) -> io::Result<usize> {
    let mut stdout = io::stdout();
    let mut output = format!("{}{}", clear::All, cursor::Goto(1, 1));
    let mut length: usize = 0usize;
    let mut entries = fs::read_dir(".").await?.enumerate();

    while let Some((i, entry_result)) = entries.next().await {
        output.push_str(format_output(entry_result?, pos == i).await?.as_str());
        length = i;
    }
    stdout.write_all(output.as_bytes()).await?;
    Ok(length)
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

    output.push_str(
        format!(
            "{}{}{}\r\n",
            name,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        ).as_str()
    );
    Ok(output)
}
