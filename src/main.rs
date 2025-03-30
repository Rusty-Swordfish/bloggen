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
use models::{Post, Frontmatter};

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


// // Bring models and structs into scope
// mod models; // Defines Post, Frontmatter, NavigationLink

// // Bring the parser logic into scope
// mod parser; // Defines parse_markdown()

// // Bring templates into scope
// mod templates; // Defines Askama templates like PostTemplate, IndexTemplate, TagTemplate

// // Bring rendering modules into scope
// mod render {
//     pub mod post;    // Defines render_post()
//     pub mod index;   // Defines render_index()
//     pub mod tags;    // Defines render_tags()
//     pub mod rss;     // Defines render_rss()
//     pub mod sitemap; // Defines render_sitemap()
// }

// // Use external crates
// use anyhow::Result;
// use std::fs;

// // Use functions from render modules
// use render::{post, index, tags, rss, sitemap};

// // Use the parser module to process markdown files
// use parser::parse_markdown;

// // Bring models into scope
// use models::{Post, Frontmatter};


// fn main() -> Result<()> {
//     let posts_dir = "posts";
//     let output_dir = "dist";
    
//     // Create output directory
//     fs::create_dir_all(output_dir)?;

//     // Process all markdown files
//     let mut posts = Vec::new();
//     for entry in fs::read_dir(posts_dir)? {
//         let path = entry?.path();
//         if path.extension().and_then(|s| s.to_str()) == Some("md") {
//             let content = fs::read_to_string(&path)?;
//             let post = parse_markdown(&content)?;

//             // Skip draft posts
//             if !post.frontmatter.draft {
//                 posts.push(post); // Add the post to the vector first
//             }
//         }
//     }

//     // Sort posts by date (newest first)
//     posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

//     // Render individual posts
//     for post in &posts {
//         render_post(post, &posts, output_dir)?; // Render after posts are sorted
//     }

//     // Generate index
//     render_index(&posts, output_dir)?;
//     // Generate RSS feed
//     render_rss(&posts, output_dir)?;

//     render_sitemap(&posts, output_dir)?;

//     Ok(())
// }