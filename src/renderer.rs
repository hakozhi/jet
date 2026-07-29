use std::collections::HashMap;
use chrono::{Datelike};
use minijinja::{Environment, context};

use crate::article::{Article, Articles};

#[derive(serde::Serialize)]
struct YearArchive {
    articles: Articles,
}

type YearArchives = HashMap<i32, YearArchive>;


#[derive(Clone)]
pub struct Renderer {
    env: Environment<'static>,
}

impl Renderer {
    pub fn new(article_template: &'static str, homepage_template: &'static str) -> Self {
        let mut env: Environment<'static> = Environment::new();
        env.add_template("homepage", homepage_template).unwrap();
        env.add_template("article", article_template).unwrap();

        Renderer {
            env: env,
        }
    }
    pub fn render_article(&self, article: &Article) -> String {
        let tmpl = self.env.get_template("article").unwrap();
        tmpl.render(context! { title => article.title, content => article.content, description => article.description })
            .unwrap()

    }
    pub fn render_homepage(&self, articles: Articles, is_production: bool) -> String {
        let year_archives = create_year_archives(articles, is_production);
        let mut years = year_archives.keys().collect::<Vec<&i32>>();
        years.sort();
        years.reverse();
        let tmpl = self.env.get_template("homepage").unwrap();

        tmpl.render(context! { years => years, year_archives => year_archives }).unwrap()
    }
}

fn create_year_archives(articles: Articles, is_production: bool) -> YearArchives {
    let mut year_archives: YearArchives = HashMap::new();

    for article in articles {
        if !article.draft || !is_production {
            let year = article.date.year();
            if let Some(year_archive) = year_archives.get_mut(&year) {
                year_archive.articles.push(article);
            } else {
                year_archives.insert(
                    year,
                    YearArchive {
                        articles: vec![article],
                    },
                );
            }
        }
    }

    for year_archive in year_archives.values_mut() {
        year_archive.articles.sort_by(|a, b| b.date.cmp(&a.date));
    }

    return year_archives;
}
