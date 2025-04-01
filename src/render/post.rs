use std::{fs::File, io::Write, path::Path};

use anyhow::Result;
use askama::Template;
use crate::{
    models::{Post, NavigationLink},
    templates::PostTemplate
};

pub fn render_post(post: &Post, all_posts: &[Post], output_dir: &str) -> Result<()> {
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
        frontmatter: &post.frontmatter,
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
