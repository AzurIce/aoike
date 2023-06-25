use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use pathdiff::diff_paths;
use pulldown_cmark::{html, Options, Parser};
use serde::{Serialize, Deserialize};
use crate::commands::build::POST_DIR;

pub struct Post {
    pub entry: DirEntry,
    pub document: Option<String>,
}

impl Post {
    pub fn from_entry(entry: DirEntry) -> Post {
        return Post {
            entry,
            document: None,
        }
    }
    pub fn file_path(&self) -> PathBuf {
        self.entry.path()
    }
    pub fn file_name(&self) -> String {
        self.entry.file_name().to_str().expect("Failed to get name").to_string()
    }
    pub fn post_dir(&self) -> PathBuf {
        let mut post_dir = self.file_path().parent().unwrap().to_path_buf();
        while !post_dir.ends_with(POST_DIR) {
            post_dir.pop();
        }
        post_dir
    }
    pub fn site_dir(&self) -> PathBuf {
        let mut site_dir = self.post_dir();
        site_dir.pop();
        site_dir
    }
    pub fn dst_path(&self) -> PathBuf {
        let dst_path = self.site_dir()
            .join(diff_paths(self.file_path(), self.post_dir()).unwrap());
        let dst_path = dst_path.with_extension("html");
        dst_path
    }
    pub fn url(&self) -> String {
        let url = diff_paths(&self.dst_path(), &self.post_dir()).unwrap()
            .to_str().unwrap().to_string();
        url
    }
    pub fn content(&self) -> String {
        if let Some(content) = &self.document {
            content.to_string()
        } else {
            fs::read_to_string(self.file_path()).expect(&*format!("Failed to read file {}", self.file_name()))
        }
    }
    pub fn rendered_content(&self) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_TASKLISTS);
        let content = self.content();
        let parser = Parser::new_ext(&content, options);
        let mut html = String::new();
        html::push_html(&mut html, parser);
        html
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostData {
    pub url: String,
    pub raw_content: String,
    pub rendered_content: String,
}

impl PostData {
    pub fn from_post(post: &Post) -> PostData {
        PostData {
            url: post.url(),
            raw_content: post.content(),
            rendered_content: post.rendered_content(),
        }
    }
}