pub mod app;
pub mod components {
    pub mod giscus;
}

use std::{fmt::Debug, sync::Arc};

use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct Site {
    pub blogs: Vec<BlogData>,
}

#[derive(Clone)]
pub struct RsxFn(pub Arc<dyn Fn() -> Element + Send + Sync>);

impl AsRef<dyn Fn() -> Element + Send + Sync + 'static> for RsxFn {
    fn as_ref(&self) -> &(dyn Fn() -> Element + Send + Sync + 'static) {
        self.0.as_ref()
    }
}

impl RsxFn {
    pub fn new(f: impl Fn() -> Element + Send + Sync + 'static) -> Self {
        Self(Arc::new(f))
    }
}

impl PartialEq for RsxFn {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Debug for RsxFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContentFn")
            .field("ptr", &Arc::as_ptr(&self.0))
            .finish()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BlogData {
    pub title: String,
    pub slug: String,
    pub summary_rsx: RsxFn,
    pub content_rsx: RsxFn,
}

impl Debug for BlogData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlogMeta")
            .field("title", &self.title)
            .field("content", &"Fn() -> Element")
            .finish()
    }
}