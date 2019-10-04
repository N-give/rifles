use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use termion::color;

pub async fn print_dir_contents(pos: usize) -> io::Result<()> {
    let mut stdout = io::stdout();

    let contents = match get_dir_contents(".").await {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to get directory contents: {}", e);
            Vec::new()
        }
    };
    for content in contents
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if i == pos {
                format!(
                    "{}{}{}{}{}\n",
                    color::Fg(color::Black),
                    color::Bg(color::Blue),
                    n.file_name().into_string().unwrap(),
                    color::Fg(color::Blue),
                    color::Bg(color::Reset)
                )
            } else {
                format!(
                    "{}{}{}{}{}\n",
                    color::Fg(color::Blue),
                    color::Bg(color::Black),
                    n.file_name().into_string().unwrap(),
                    color::Fg(color::Blue),
                    color::Bg(color::Reset)
                )
            }
        })
        .into_iter() {
        stdout.write_all(&content.as_bytes()).await?;
    };
    Ok(())
}

async fn get_dir_contents(dir: &str) -> io::Result<Vec<fs::DirEntry>> {
    Ok(fs::read_dir(dir)
        .await?
        .filter_map(|r_entry| r_entry.ok())
        .collect()
        .await)
}
