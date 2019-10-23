use async_std::task;

mod contents;
mod input;

fn main() {
    task::block_on(async {
        contents::print_dir_contents(0)
            .await
            .expect("Failed to write directory contents");

        let dir = input::move_pos().await.expect("Failed to get next position");

        match dir {
            input::Dir::DN => contents::print_dir_contents(1)
                .await
                .expect("Failed to write directory contents"),

            _ => eprintln!("choose a different direction, dumby"),
        }
    });
}
