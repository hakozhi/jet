use std::fs;
use std::path;
use std::path::Path;
use crate::error::JetError;
use crate::error::Result;
use crate::article::Articles;
use crate::renderer::Renderer;

pub fn create_homepage_html_file(articles: Articles, output_dir_path: &Path, renderer: &Renderer, is_production: bool) -> Result<()> {
    if !path::Path::new(&output_dir_path).is_dir() {
        fs::create_dir(&output_dir_path).unwrap();
    }

    let homepage_html_filename = Path::new(output_dir_path).join("index.html");
    let html = renderer.render_homepage(articles, is_production);

    match fs::write(homepage_html_filename, html) {
        Ok(_) => Ok(()),
        Err(_) => Err(JetError::FailedToCreateHomepageFile)
    }
}
