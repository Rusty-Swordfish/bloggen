use anyhow::{Context, Result};
use askama::Template;
use pulldown_cmark::{html::push_html, Options, Parser};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Deserialize, PartialEq)]
struct Frontmatter {
    pub title: String,
    pub date: String,
    pub slug: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, PartialEq)]
struct Post {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub pub_date: DateTime<Utc>,
}

// Update NavigationLink to be a simple struct with public fields
#[derive(Debug)]
struct NavigationLink<'a> {
    pub title: &'a str,
    pub slug: &'a str,
}

// Keep your existing PostTemplate struct
#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    title: &'a str,
    date: &'a str,
    content: &'a str,
    previous_post: Option<NavigationLink<'a>>,
    next_post: Option<NavigationLink<'a>>,
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
                posts.push(post); // Add the post to the vector first
            }
        }
    }

    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    // Render individual posts
    for post in &posts {
        render_post(post, &posts, output_dir)?; // Render after posts are sorted
    }

    // Generate index
    render_index(&posts, output_dir)?;
    // Generate RSS feed
    render_rss(&posts, output_dir)?;

    render_sitemap(&posts, output_dir)?;

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

    // Convert date string to DateTime
    // Replace deprecated method
    let pub_date = DateTime::parse_from_str(
        &format!("{} 00:00:00 +0000", frontmatter.date),
        "%Y-%m-%d %H:%M:%S %z"
    )?
    .with_timezone(&Utc);

    Ok(Post {
        frontmatter,
        content: html,
        pub_date,
    })
}

fn render_post(post: &Post, all_posts: &[Post], output_dir: &str) -> Result<()> {
    let index = all_posts.iter().position(|p| p == post).unwrap();

    let previous_post = all_posts.get(index + 1).map(|p| NavigationLink {
        title: &p.frontmatter.title,
        slug: &p.frontmatter.slug,
    });

    let next_post = if index > 0 {
        all_posts.get(index - 1).map(|p| NavigationLink {
            title: &p.frontmatter.title,
            slug: &p.frontmatter.slug,
        })
    } else {
        None
    };

    let template = PostTemplate {
        title: &post.frontmatter.title,
        date: &post.frontmatter.date,
        content: &post.content,
        previous_post,
        next_post,
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

#[derive(Template)]
#[template(path = "rss.xml", escape = "none")]
struct RssTemplate<'a> {
    posts: &'a [&'a Post],  // Change to slice of references
}

// Update RSS rendering
fn render_rss(posts: &[Post], output_dir: &str) -> Result<()> {
    let posts_refs: Vec<&Post> = posts.iter().collect();
    let template = RssTemplate {
        posts: &posts_refs,
    };
    fs::write(Path::new(output_dir).join("feed.xml"), template.render()?)?;
    Ok(())
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
struct SitemapTemplate {
    urls: Vec<SitemapUrl>,
}

// Add this new helper struct
struct SitemapUrl {
    pub loc: String,
    pub lastmod: String,
}

fn render_sitemap(posts: &[Post], output_dir: &str) -> Result<()> {
    let base_url = "https://example.com/"; // Replace with your actual domain
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
            lastmod: homepage_lastmod.to_rfc3339(), // You can use a different date if needed
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
            lastmod: homepage_lastmod.to_rfc3339(), // You can use a different date if needed
        });
    }

    // Render sitemap
    let template = SitemapTemplate { urls };
    let output_path = Path::new(output_dir).join("sitemap.xml");
    fs::write(output_path, template.render()?)?;

    Ok(())
}
