use async_std::io;
use async_std::prelude::*;

pub enum Dir {
    UP,
    DN,
    LT,
    RT
}

pub async fn move_pos() -> io::Result<Dir> {
    let mut uin = String::new();
    io::stdin().read_line(&mut uin).await?;

    match uin.trim() {
        "h" => Ok(Dir::RT),
        "j" => Ok(Dir::DN),
        "k" => Ok(Dir::UP),
        "l" => Ok(Dir::LT),
        _ => unimplemented!(),
    }
}
