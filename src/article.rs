use chrono::{NaiveDate};
use minijinja::{context, Environment};
use markdown_frontmatter;
use std::io;
use std::fs;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use crate::helper;

#[derive(serde::Serialize)]
pub struct Article {
    pub title: String,
    pub date: NaiveDate,
    pub content: String,
    pub slug: String,
    pub draft: bool,
    pub description: String,
}

impl Article {
    pub fn from_file(path: &Path) -> Article {
        let content = helper::read_file_content(path);
        let (frontmatter, body) = markdown_frontmatter::parse::<Frontmatter>(&content).unwrap();
        let compile_options = markdown::CompileOptions {
            allow_dangerous_html: true,
            ..markdown::CompileOptions::default()
        };
        let options = markdown::Options {
            compile: compile_options,
            ..markdown::Options::gfm()
        };

        Article {
            title: frontmatter.title,
            date: chrono::NaiveDate::parse_from_str(&frontmatter.date, "%Y-%m-%d")
                .expect("The format of date is incorrect."),
            content: markdown::to_html_with_options(body, &options).unwrap(),
            slug: frontmatter.slug,
            draft: frontmatter.draft,
            description: frontmatter.description
        }
    }

    pub fn to_html(&self, template: &str) -> String {
        let mut env = Environment::new();
        env.add_template("article", template).unwrap();
        let tmpl = env.get_template("article").unwrap();

        tmpl.render(context! { title => self.title, content => self.content, description => self.description })
            .unwrap()
    }
}

pub type Articles = Vec<Article>;

#[derive(serde::Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    slug: String,
    draft: bool,
    description: String,
}

pub fn get_articles(articles_dir: &Path) -> Articles {
    let filepaths = get_article_filepaths(&articles_dir).unwrap();
    let articles: Articles = filepaths.into_iter()
        .map(|path| Article::from_file(&path))
        .collect();

    return articles;
}

pub fn create_article_html_file(
    article: &Article,
    article_template_path: &Path,
    output_dir: &Path,
) -> io::Result<()> {
    if !path::Path::new(&output_dir).is_dir() {
        fs::create_dir(&output_dir)?;
    }

    let mut output_dir_path = path::PathBuf::from(output_dir);
    output_dir_path.push(&(article.slug.clone() + ".html"));

    fs::write(
        output_dir_path,
        article.to_html(&helper::read_file_content(article_template_path))
    )?;
    return Ok(())
}

fn get_article_filepaths(article_directory: &Path) -> io::Result<Vec<PathBuf>> {
    let mut article_filepaths: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(article_directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_files = get_article_filepaths(&path)?;
            article_filepaths.extend(sub_files);
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    article_filepaths.push(path);
                }
            }
        }
    }

    return Ok(article_filepaths);
}

