use std::path::{Path, PathBuf};

use dioxus::prelude::*;

#[derive(Clone)]
pub struct AoikeContext {
    pub root_dir: PathBuf,
    pub cmark_options: pulldown_cmark::Options,
}

#[derive(Debug)]
pub struct Site {
    pub root_dir: PathBuf,
    pub blogs: Vec<BlogMeta>,
}

#[derive(Debug)]
pub struct Blogs {}

#[derive(Props, PartialEq, Clone, Debug)]
pub struct BlogMeta {
    pub title: String,
    pub document: String,
}

#[component]
pub fn BlogCard(blog: BlogMeta) -> Element {
    tracing::info!("Rendered with blog: {blog:?}");
    rsx! {
        div {
            "{blog.title}"
            "{blog.document}"
        }
    }
}
