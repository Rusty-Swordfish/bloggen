use anyhow::Result;
use pulldown_cmark::{Options, Parser, html::push_html};
use serde_yaml;
use chrono::{DateTime, Utc, TimeZone};
use crate::models::{Frontmatter, Post};

pub fn parse_markdown(content: &str) -> Result<Post> {
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
