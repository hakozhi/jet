use std::path::Path;

use crate::article::Articles;
use crate::error::{JetError, Result};
use crate::{article, helper};
use serde;
use toml;
use url::Url;

pub struct Site {
    pub config: SiteConfig,
    pub articles: Articles,
}

pub struct SiteConfig {
    pub title: String,
    pub base_url: Url,
    pub description: String,
}

#[derive(serde::Deserialize)]
struct Config {
    title: String,
    base_url: String,
    description: String,
}

impl Site {
    pub fn new(config_path: &Path, articles_dir: &Path) -> Result<Site> {
        let Config {
            title,
            base_url,
            description,
        } = Self::read_blog_config(config_path)?;
        let base_url = match Url::parse(&base_url) {
            Ok(url) => url,
            Err(_) => return Err(JetError::InvalidBaseURL),
        };
        let config = SiteConfig {
            title,
            base_url,
            description,
        };

        Ok(Site {
            config,
            articles: article::get_articles(articles_dir),
        })
    }

    fn read_blog_config(path: &Path) -> Result<Config> {
        let toml_content = helper::read_file_content(path);

        match toml::from_str(&toml_content) {
            Ok(config) => Ok(config),
            Err(_) => Err(JetError::IncompleteSiteConfig),
        }
    }
}
