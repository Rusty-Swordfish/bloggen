use askama::Template;
use crate::models::{Frontmatter, NavigationLink, Page, Post, SitemapUrl};
use crate::filters;

#[derive(Template)]
#[template(path = "post.html")]
pub(crate) struct PostTemplate<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub content: &'a str,
    pub frontmatter: &'a Frontmatter,
    pub previous_post: Option<NavigationLink<'a>>,
    pub next_post: Option<NavigationLink<'a>>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate<'a> {
    pub posts: &'a [Post],
}

#[derive(Template)]
#[template(path = "tags.html")]
pub(crate) struct TagTemplate<'a> {
    pub tag: &'a str,
    pub posts: &'a [&'a Post],
}

#[derive(Template)]
#[template(path = "rss.xml", escape = "none")]
pub(crate) struct RssTemplate<'a> {
    pub posts: &'a [&'a Post],
}

#[derive(Template)]
#[template(path = "sitemap.xml")]
pub(crate) struct SitemapTemplate {
    pub urls: Vec<SitemapUrl>,
}

// Default template
#[derive(Template)]
#[template(path = "page.html")]
pub struct DefaultPageTemplate<'a> {
    pub page: &'a Page,
}

// Add more templates as needed
#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactPageTemplate<'a> {
    pub page: &'a Page,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutPageTemplate<'a> {
    pub page: &'a Page,
}

#[derive(Template)]
#[template(path = "category.html")]
pub struct CategoryTemplate<'a> {
    pub category: &'a str,
    pub posts: &'a [&'a Post],
}