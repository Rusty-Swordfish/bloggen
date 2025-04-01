use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use askama::Template;
use crate::models::Page;
use crate::{
    templates::DefaultPageTemplate,
    templates::ContactPageTemplate,
    templates::AboutPageTemplate
};

pub fn render_page(page: &Page, output_dir: &str) -> Result<()> {
    let output_path = Path::new(output_dir).join(&page.slug).with_extension("html");
    let mut file = File::create(output_path)?;
    
    // Use the template field to determine which template to render
    let content = match page.template.as_str() {
        "default" | "" => DefaultPageTemplate { page }.render()?,
        "contact" => ContactPageTemplate { page }.render()?,
        "about" => AboutPageTemplate { page }.render()?,
        // Add more template types as needed
        _ => return Err(anyhow!("Unknown template type: {}", page.template)),
    };
    
    file.write_all(content.as_bytes())?;
    Ok(())
}
