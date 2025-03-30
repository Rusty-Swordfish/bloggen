use askama::Template;
use crate::models::{NavigationLink, Post, SitemapUrl};

#[derive(Template)]
#[template(path = "post.html")]
pub(crate) struct PostTemplate<'a> {  // Needs pub(crate) if used in render/post.rs
    pub title: &'a str,
    pub date: &'a str,
    pub content: &'a str,
    pub previous_post: Option<NavigationLink<'a>>,
    pub next_post: Option<NavigationLink<'a>>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate<'a> {  // Add pub(crate) if used in render/index.rs
    pub posts: &'a [Post],
}

#[derive(Template)]
#[template(path = "tags.html")]
pub(crate) struct TagTemplate<'a> {    // Add pub(crate) if used in render/tags.rs
    pub tag: &'a str,
    pub posts: &'a [&'a Post],
}

#[derive(Template)]
#[template(path = "rss.xml", escape = "none")]
pub(crate) struct RssTemplate<'a> {    // Add pub(crate) if used in render/rss.rs
    pub posts: &'a [&'a Post],
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub(crate) struct SitemapTemplate {    // Add pub(crate) if used in render/sitemap.rs
    pub urls: Vec<SitemapUrl>,
}
