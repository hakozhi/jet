use std::path::Path;

use serde;
use toml;
use crate::error::{JetError, Result};
use crate::{article, helper};
use crate::article::Articles;

pub struct Site {
    pub config: Config,
    pub articles: Articles
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub title: String,
    pub base_url: String,
    pub description: String,
}

impl Site {
    pub fn new(config_path: &Path, articles_dir: &Path) -> Result<Site> {
        Ok(Site {
            config: Site::read_blog_config(config_path)?,
            articles: article::get_articles(articles_dir),
        })
    }

    fn read_blog_config(path: &Path) -> Result<Config> {
        let toml_content = helper::read_file_content(path);

        match toml::from_str(&toml_content) {
            Ok(config) => Ok(config),
            Err(_) => Err(JetError::IncompleteSiteConfig)
        }
    }
}
