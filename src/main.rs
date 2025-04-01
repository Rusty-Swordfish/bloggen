mod models;
mod parser;
mod templates;
mod filters;
mod render {
    pub mod post;
    pub mod index;
    pub mod tags;
    pub mod rss;
    pub mod sitemap;
    pub mod page;
}


use anyhow::Result;
use std::fs;
use parser::post::parse_markdown;
use parser::page::parse_page;
use render::{post, page, index, tags, rss, sitemap};

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

    let pages_dir = "pages";
    let mut pages = Vec::new();

    // Process all page files
    for entry in fs::read_dir(pages_dir)? {
        let path = entry?.path();
        println!("{:?}", path);
        if path.is_file() {
            let page = parse_page(&path)?;
            pages.push(page);
        }
    }

    // Render pages
    for page in &pages {
        page::render_page(page, output_dir)?;
    }

    // Generate index
    index::render_index(&posts, output_dir)?;

    // Generate tag pages
    tags::render_tags(&posts, output_dir)?;

    // Generate RSS feed
    rss::render_rss(&posts, output_dir)?;

    // Generate sitemap
    sitemap::render_sitemap(&posts, output_dir, "https://yourdomain.com/")?;

    Ok(())
}