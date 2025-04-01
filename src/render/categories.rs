use anyhow::Result;
use std::fs;
use std::path::Path;
use askama::Template;
use crate::models::Post;
use crate::templates::CategoryTemplate;

pub fn render_categories(posts: &[Post], output_dir: &str) -> Result<()> {
    use std::collections::HashMap;

    let mut categories: HashMap<&str, Vec<&Post>> = HashMap::new();

    // Group posts by categories
    for post in posts {
        if !post.frontmatter.category.is_empty() {
            categories
                .entry(post.frontmatter.category.as_str())
                .or_insert_with(Vec::new)
                .push(post);
        }
    }

    // Create output directory for categories
    let categories_dir = Path::new(output_dir).join("categories");
    fs::create_dir_all(&categories_dir)?;

    // Render each category page
    for (category, posts) in categories {
        let template = CategoryTemplate {
            category,
            posts: &posts,
        };

        let output_path = categories_dir.join(format!("{}.html", category));
        fs::write(output_path, template.render()?)?;
    }

    Ok(())
}
