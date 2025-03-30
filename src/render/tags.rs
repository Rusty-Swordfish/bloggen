use anyhow::Result;
use std::fs;
use std::path::Path;
use askama::Template;
use crate::models::Post;
use crate::templates::TagTemplate;

pub fn render_tags(posts: &[Post], output_dir: &str) -> Result<()> {
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