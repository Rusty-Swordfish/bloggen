use anyhow::Result;
use std::fs::File;
use std::io::Write;          // Needed for the `write_all` method
use std::path::Path;         // Needed for filesystem path handling
use askama::Template;        // Needed for the `render` method
use crate::models::Post;     // Needed for the `Post` struct
use crate::templates::IndexTemplate; // Needed for the `IndexTemplate` struct

pub fn render_index(posts: &[Post], output_dir: &str) -> Result<()> {
    let template = IndexTemplate { posts };
    let output_path = Path::new(output_dir).join("index.html");
    
    let mut file = File::create(output_path)?;
    file.write_all(template.render()?.as_bytes())?;
    Ok(())
}
