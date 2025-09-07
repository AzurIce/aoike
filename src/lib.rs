pub mod app;

use std::{fmt::Debug, sync::Arc};

use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct Site {
    pub blogs: Vec<BlogMeta>,
}

#[derive(Props, Clone)]
pub struct BlogMeta {
    pub title: String,
    pub content_fn: Arc<dyn Fn() -> Element + Send + Sync>,
}

impl PartialEq for BlogMeta {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && Arc::ptr_eq(&self.content_fn, &other.content_fn)
    }
}

impl Debug for BlogMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlogMeta")
            .field("title", &self.title)
            .field("content", &"Fn() -> Element")
            .finish()
    }
}