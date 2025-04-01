use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use askama::Template;
use crate::models::Post;
use crate::templates::IndexTemplate;

pub fn render_index(posts: &[Post], output_dir: &str) -> Result<()> {
    let template = IndexTemplate { posts };
    let output_path = Path::new(output_dir).join("index.html");
    
    let mut file = File::create(output_path)?;
    file.write_all(template.render()?.as_bytes())?;
    Ok(())
}
