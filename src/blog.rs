use std::path::Path;

use serde;
use toml;
use crate::{article, helper};
use crate::article::Articles;

pub struct Blog {
    pub config: Config,
    pub articles: Articles
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub title: String,
    pub base_url: String,
    pub description: String,
}

impl Blog {
    pub fn new(config_path: &Path, articles_dir: &Path) -> Blog {
        Blog {
            config: Blog::read_blog_config(config_path),
            articles: article::get_articles(articles_dir),
        }
    }

    fn read_blog_config(path: &Path) -> Config {
        let toml_content = helper::read_file_content(path);

        match toml::from_str(&toml_content) {
            Ok(config) => config,
            Err(_) => panic!("jet.toml are incomplete")
        }
    }
}
