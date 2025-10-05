pub use aoike;

pub mod app;
pub mod components {
    pub mod giscus;
}

#[cfg(feature = "build")]
pub mod build;

use std::sync::Arc;

use dioxus::prelude::*;
pub use time;
use time::UtcDateTime;

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

#[derive(Props, Clone, PartialEq)]
pub struct PostData {
    pub title: String,
    pub slug: String,
    pub summary_rsx: RsxFn,
    pub content_rsx: RsxFn,
    pub created: UtcDateTime,
    pub updated: UtcDateTime,
}

#[derive(Clone)]
pub struct Site {
    pub posts: &'static [PostData],
    pub index: &'static PostData,
}
