use rss::{ChannelBuilder, Item, ItemBuilder};
use chrono::{Utc, TimeZone};
use url::Url;
use crate::site::Site;
use crate::article::{Article, Articles};
use std::fs;
use std::path::Path;

pub fn create_rss_xml(blog: &Site, output_dir: &Path) {
    let rss_content = create_rss_content(blog);

    fs::write(output_dir.join("rss.xml"), rss_content).unwrap();
}

fn create_rss_content(blog: &Site) -> String {
    let channel = ChannelBuilder::default()
        .title(&blog.config.title)
        .link(format!("{}rss.xml", &blog.config.base_url))
        .description(&blog.config.description)
        .items(create_article_items(&blog.config.base_url, &blog.articles))
        .build();

    return channel.to_string();
}

fn create_article_items(base_url: &Url, articles: &Articles) -> Vec<Item> {
    let mut article_items: Vec<Item> = vec![];

    for article in articles {
        if !article.draft {
            article_items.push(make_article_item(base_url, article));
        }
    }

    return article_items;
}

fn make_article_item(base_url: &Url, article: &Article) -> Item {
    let pub_date = Utc.from_utc_datetime(&article.date.and_hms_opt(12, 0, 0).unwrap()).to_rfc2822();
    let link = base_url.join(&format!("/posts/{}", article.slug)).unwrap();

    return ItemBuilder::default()
        .title(article.title.clone())
        .link(link.to_string())
        .description(article.description.clone())
        .content(article.content.clone())
        .pub_date(pub_date)
        .build();
}
