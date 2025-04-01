use anyhow::Result;
use std::fs;
use std::path::Path;
use askama::Template;
use crate::models::Post;
use crate::templates::RssTemplate;

pub fn render_rss(posts: &[Post], output_dir: &str) -> Result<()> {
    let posts_refs: Vec<&Post> = posts.iter().collect();
    let template = RssTemplate {
        posts: &posts_refs,
    };
    fs::write(Path::new(output_dir).join("feed.xml"), template.render()?)?;
    Ok(())
}