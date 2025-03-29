use anyhow::{Context, Result};
use askama::Template;
use pulldown_cmark::{html::push_html, Options, Parser};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    slug: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    summary: String,  // Not optional - empty string by default
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    category: String,
}

#[derive(Debug)]
struct Post {
    frontmatter: Frontmatter,
    content: String,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    title: &'a str,
    date: &'a str,
    content: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    posts: &'a [Post],
}

#[derive(Template)]
#[template(path = "tags.html")]
struct TagTemplate<'a> {
    tag: &'a str,
    posts: &'a [&'a Post],
}



fn main() -> Result<()> {
    let posts_dir = "posts";
    let output_dir = "dist";
    
    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Process all markdown files
    let mut posts = Vec::new();
    for entry in fs::read_dir(posts_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path)?;
            let post = parse_markdown(&content)?;
    
            // Skip draft posts
            if !post.frontmatter.draft {
                render_post(&post, output_dir)?;
                posts.push(post);
            }
        }
    }

    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    // Generate index
    render_index(&posts, output_dir)?;

    Ok(())
}

fn parse_markdown(content: &str) -> Result<Post> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        anyhow::bail!("Invalid frontmatter format");
    }

    let frontmatter: Frontmatter = serde_yaml::from_str(parts[1])?;
    let markdown = parts[2];
    
    // Convert markdown to HTML
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html = String::new();
    push_html(&mut html, parser);

    Ok(Post {
        frontmatter,
        content: html,
    })
}

fn render_post(post: &Post, output_dir: &str) -> Result<()> {
    let template = PostTemplate {
        title: &post.frontmatter.title,
        date: &post.frontmatter.date,
        content: &post.content,
    };

    let output_path = Path::new(output_dir)
        .join(&post.frontmatter.slug)
        .with_extension("html");
    
    let mut file = File::create(output_path)?;
    file.write_all(template.render()?.as_bytes())?;
    Ok(())
}

fn render_index(posts: &[Post], output_dir: &str) -> Result<()> {
    let template = IndexTemplate { posts };
    let output_path = Path::new(output_dir).join("index.html");
    
    let mut file = File::create(output_path)?;
    file.write_all(template.render()?.as_bytes())?;
    Ok(())
}

fn render_tags(posts: &[Post], output_dir: &str) -> Result<()> {
    use std::collections::HashMap;

    let mut tags: HashMap<&str, Vec<&Post>> = HashMap::new();

    // Group posts by tags
    for post in posts {
        for tag in &post.frontmatter.tags {
            tags.entry(tag.as_str())
                .or_insert_with(Vec::new)
                .push(post);
        }
    }

    // Create output directory for tags
    let tags_dir = Path::new(output_dir).join("tags");
    fs::create_dir_all(&tags_dir)?;

    // Render each tag page
    for (tag, posts) in tags {
        let template = TagTemplate {
            tag,
            posts: &posts,
        };

        let output_path = tags_dir.join(format!("{}.html", tag));
        fs::write(output_path, template.render()?)?;
    }

    Ok(())
}
