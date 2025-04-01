use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::models::Page;

pub fn parse_page(path: &Path) -> Result<Page> {
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    match extension {
        "md" => parse_markdown_page(path),
        "json" => parse_json_page(path),
        _ => Err(anyhow::anyhow!("Unsupported file format: {}", extension)),
    }
}

fn parse_markdown_page(path: &Path) -> Result<Page> {
    let content = fs::read_to_string(path)?;
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid frontmatter format in file {:?}", path));
    }
    
    // Define a temporary struct for the page frontmatter
    #[derive(Debug, Deserialize)]
    struct PageFrontmatter {
        pub title: String,
        pub slug: String,
        pub template: String,
        #[serde(default)]
        pub custom_data: Option<serde_json::Value>,
    }
    
    // Parse the frontmatter
    let frontmatter: PageFrontmatter = serde_yaml::from_str(parts[1])?;
    let markdown = parts[2];
    
    // Convert markdown to HTML
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    // Create the complete Page
    Ok(Page {
        title: frontmatter.title,
        slug: frontmatter.slug,
        content: html,
        template: frontmatter.template,
        custom_data: frontmatter.custom_data,
    })
}

fn parse_json_page(path: &Path) -> Result<Page> {
    let content = fs::read_to_string(path)?;
    match serde_json::from_str(&content) {
        Ok(page) => Ok(page),
        Err(e) => Err(anyhow::anyhow!("Error parsing JSON file {:?}: {}", path, e)),
    }
}