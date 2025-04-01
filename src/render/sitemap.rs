use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use askama::Template;
use crate::models::{Page, Post, SitemapUrl};
use crate::templates::SitemapTemplate;

pub fn render_sitemap(posts: &[Post], pages: &[Page], output_dir: &str, base_url: &str) -> Result<()> {
    let mut urls = Vec::new();

    // Add homepage
    let homepage_lastmod = posts
        .iter()
        .map(|p| p.pub_date)
        .max()
        .unwrap_or_else(|| Utc::now());
    urls.push(SitemapUrl {
        loc: format!("{}", base_url),
        lastmod: homepage_lastmod.to_rfc3339(),
    });

    // Add blog posts
    for post in posts {
        urls.push(SitemapUrl {
            loc: format!("{}posts/{}.html", base_url, post.frontmatter.slug),
            lastmod: post.pub_date.to_rfc3339(),
        });
    }

    // Add pages
    for page in pages {
        urls.push(SitemapUrl {
            loc: format!("{}{}.html", base_url, page.slug),
            lastmod: homepage_lastmod.to_rfc3339(), // Using latest post date as page lastmod
        });
    }

    // Add tags
    let mut tags: HashMap<&str, Vec<&Post>> = HashMap::new();
    for post in posts {
        for tag in &post.frontmatter.tags {
            tags.entry(tag.as_str()).or_insert_with(Vec::new).push(post);
        }
    }
    for tag in tags.keys() {
        urls.push(SitemapUrl {
            loc: format!("{}tags/{}.html", base_url, tag),
            lastmod: homepage_lastmod.to_rfc3339(),
        });
    }

    // Add categories
    let mut categories: HashMap<&str, Vec<&Post>> = HashMap::new();
    for post in posts {
        if !post.frontmatter.category.is_empty() {
            categories
                .entry(post.frontmatter.category.as_str())
                .or_insert_with(Vec::new)
                .push(post);
        }
    }
    for category in categories.keys() {
        urls.push(SitemapUrl {
            loc: format!("{}categories/{}.html", base_url, category),
            lastmod: homepage_lastmod.to_rfc3339(),
        });
    }

    // Render sitemap
    let template = SitemapTemplate { urls };
    let output_path = Path::new(output_dir).join("sitemap.xml");
    fs::write(output_path, template.render()?)?;

    Ok(())
}