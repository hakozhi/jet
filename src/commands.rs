use crate::article::create_article_html_file;
use crate::error::JetError;
use crate::error::Result;
use crate::generate;
use crate::helper;
use crate::renderer::Renderer;
use crate::rss;
use crate::server;
use crate::site::Site;
use chrono;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use url::Url;

use clap::{Parser, Subcommand};

const DEFAULT_ARTICLE_TEMPLATE: &str = "---\n\
                                        title: \"\"\n\
                                        date: \"{date}\"\n\
                                        slug: \"{slug}\"\n\
                                        draft: true\n\
                                        description: \"\"\n\
                                        ---";

#[derive(Parser)]
#[command(version)]
pub struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Build {
        #[arg[short, long]]
        output_dir: Option<PathBuf>,
    },
    Serve,
    Create {
        article_slug: String,
    },
}

impl CLI {
    pub fn run(&self) -> Result<()> {
        let current_directory = Path::new("./");
        if !helper::check_is_root(current_directory) {
            return Err(JetError::OutsideProject);
        } else if let Command::Create { article_slug } = &self.command {
            return self.create_article(article_slug);
        }

        let config_path = Path::new("jet.toml");
        let articles_dir = Path::new("./articles");
        let site = Site::new(config_path, articles_dir)?;
        let homepage_template: String =
            match fs::read_to_string(Path::new("templates/homepage.html")) {
                Ok(template) => template,
                Err(_) => return Err(JetError::TemplateNotFound),
            };
        let article_template = match fs::read_to_string(Path::new("templates/article.html")) {
            Ok(template) => template,
            Err(_) => return Err(JetError::TemplateNotFound),
        };
        let renderer = Renderer::new(article_template.leak(), homepage_template.leak());

        match &self.command {
            Command::Build { output_dir } => {
                if let Some(output_dir) = output_dir {
                    self.build_site(output_dir, &site, renderer)
                } else {
                    self.build_site(Path::new("public/"), &site, renderer)
                }
            }
            Command::Serve => self.serve(&site, renderer),
            Command::Create { article_slug: _ } => unreachable!(),
        }
    }

    fn build_site(&self, output_dir: &Path, site: &Site, renderer: Renderer) -> Result<()> {
        generate::create_homepage_html_file(site.articles.clone(), output_dir, &renderer, true)?;

        for article in &site.articles {
            if !article.draft {
                let output_dir_path = Path::new(&output_dir).join("posts/");
                create_article_html_file(article, &renderer, &output_dir_path)?;
            }
        }

        println!("{}", output_dir.as_os_str().to_str().unwrap());
        helper::copy_assets_to_output_dir(Path::new("assets/"), output_dir);
        rss::create_rss_xml(site, output_dir);

        println!("Site was generated successfully.");
        Ok(())
    }

    fn create_article(&self, slug: &str) -> Result<()> {
        let article_content = DEFAULT_ARTICLE_TEMPLATE
            .replace(
                "{date}",
                chrono::Local::now().format("%Y-%m-%d").to_string().as_str(),
            )
            .replace("{slug}", slug);

        match fs::write(format!("articles/{}.md", slug), article_content) {
            Ok(()) => {
                println!("Create article: articles/{slug}.md");
                Ok(())
            },
            Err(_) => Err(JetError::FailedToCreateArticleFile),
        }
    }

    fn serve(&self, site: &Site, renderer: Renderer) -> Result<()> {
        let base_url = Url::parse("http://localhost:3000")
            .unwrap()
            .join(site.config.base_url.path())
            .unwrap();

        println!(
            "Web Server is available at {} (bind address 127.0.0.1) ",
            base_url
        );
        println!("Press Ctrl+C to stop");
        server::start_server(&base_url, renderer, site);

        Ok(())
    }
}
