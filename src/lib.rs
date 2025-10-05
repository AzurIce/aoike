#[cfg(feature = "build")]
pub mod build;

pub use time;
use time::UtcDateTime;

#[derive(Clone, PartialEq)]
pub struct PostData {
    pub title: String,
    pub slug: String,
    pub summary_html: String,
    pub content_html: String,
    pub created: UtcDateTime,
    pub updated: UtcDateTime,
}

#[derive(Clone)]
pub struct Site {
    pub posts: &'static [PostData],
    pub index: &'static PostData,
}
