use async_std::task;

mod contents;

fn main() {
    task::block_on(async {
        contents::print_dir_contents(0)
            .await
            .expect("Failed to write directory contents");
    });
}
