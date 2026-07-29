use clap::Parser;
use cli::CLI;
use error::JetError;
use jet::*;

fn main() {
    let cli = CLI::parse();

    if let Err(e) = cli.run() {
        let error_msg = match e {
            JetError::OutsideProject => "Error: The current directory is not a Jet project",
            JetError::TemplateNotFound => "Error: The template files are not found",
            JetError::IncompleteSiteConfig => "Error: The site config file is incomplete",
            JetError::FailedToCreateHomepageFile => "Error: Failed to create homepage file",
            JetError::FailedToCreateArticleFile => "Error: Failed to create article files",
            JetError::InvalidBaseURL => "Error: The BaseURL is invalid",
        };
        println!("{error_msg}");
    }
}
