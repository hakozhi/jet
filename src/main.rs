use clap::Parser;
use commands::CLI;
mod commands;
mod generate;
mod server;
mod helper;
mod article;
mod rss;
mod blog;

fn main() {
    let current_directory = String::from("./");

    if !helper::check_is_root(current_directory) {
        println!("Error: the current directory is not a Jet project.");
        return;
    } else {
        let cli = CLI::parse();
        cli.run();
    }
}
