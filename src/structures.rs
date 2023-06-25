use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use pulldown_cmark::{html, Options, Parser};

pub struct PostMeta {

}

pub struct Post {
    pub entry: DirEntry,
    document: Option<String>,
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