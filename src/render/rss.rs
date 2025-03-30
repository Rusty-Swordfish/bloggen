use anyhow::Result;
use std::fs;
use std::path::Path;         // Needed for filesystem path handling
use askama::Template;        // Needed for the `render` method
use crate::models::Post;     // Needed for the `Post` struct
use crate::templates::RssTemplate; // Needed for the `IndexTemplate` struct

pub fn render_rss(posts: &[Post], output_dir: &str) -> Result<()> {
    let posts_refs: Vec<&Post> = posts.iter().collect();
    let template = RssTemplate {
        posts: &posts_refs,
    };
    fs::write(Path::new(output_dir).join("feed.xml"), template.render()?)?;
    Ok(())
}