use chrono::{DateTime, Utc}; // Used for DateTime and Utc timezone in the `Post` struct
use serde::Deserialize;      // For deserializing the Frontmatter struct from YAML

#[derive(Debug, Deserialize, PartialEq)]
pub struct Frontmatter {
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
pub struct Post {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub pub_date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NavigationLink<'a> {
    pub title: &'a str,
    pub slug: &'a str,
}

pub struct SitemapUrl {
    pub loc: String,
    pub lastmod: String,
}
