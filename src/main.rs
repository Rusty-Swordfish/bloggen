mod models;
mod parser;
mod templates;
mod render {
    pub mod post;
    pub mod index;
    pub mod tags;
    pub mod rss;
    pub mod sitemap;
}

use anyhow::Result;
use std::fs;
use parser::parse_markdown;
use render::{post, index, tags, rss, sitemap};

fn main() -> Result<()> {
    let posts_dir = "posts";
    let output_dir = "dist";
    
    // Create output directory
    fs::create_dir_all(output_dir)?;

    let mut posts = Vec::new();

    // Process all markdown files
    for entry in fs::read_dir(posts_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path)?;
            let post = parse_markdown(&content)?;

            if !post.frontmatter.draft {
                posts.push(post);
            }
        }
    }

    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    // Render individual posts
    for post in &posts {
        post::render_post(post, &posts, output_dir)?;
    }

    // Generate index
    index::render_index(&posts, output_dir)?;

    // Generate tag pages
    tags::render_tags(&posts, output_dir)?;

    // Generate RSS feed
    rss::render_rss(&posts, output_dir)?;

    // Generate sitemap
    sitemap::render_sitemap(&posts, output_dir)?;

    Ok(())
}