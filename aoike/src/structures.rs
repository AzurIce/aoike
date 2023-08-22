use std::{path::PathBuf, fs};
use pulldown_cmark::{Options, Parser, html};
pub enum SourceFile {
    MarkdownFile(PathBuf),
    OtherFile(PathBuf)
}

impl SourceFile {
    pub fn from(path: PathBuf) -> SourceFile {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("md") => Self::MarkdownFile(path),
            Some(_) | None => Self::OtherFile(path),
        }
    }
}

use chrono::{DateTime, Local, NaiveDateTime};

#[derive(Debug)]
pub struct Post {
    pub title: String,
    pub draft: bool,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
    pub document: String,
}

impl Default for Post {
    fn default() -> Self {
        Post {
            title: String::new(),
            draft: false,
            create_time: DateTime::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Local::now().offset().clone()),
            update_time: DateTime::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Local::now().offset().clone()),
            document: String::new()
        }
    }
}

impl Post {
    pub fn from_path(path: &PathBuf) -> Self {
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let title = filename.clone();
        let document = fs::read_to_string(path).and_then(|content| {
            
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_TASKLISTS);
            let parser = Parser::new_ext(&content, options);
            
            let mut html = String::new();
            html::push_html(&mut html, parser);
            Ok(html)
        }).unwrap();

        Post {
            title,
            document,
            ..Default::default()
        }
    }
}

// impl PostData {
//     pub fn from_post(post: &Post) -> PostData {
//         PostData {
//             url: post.url(),
//             raw_content: post.content(),
//             rendered_content: post.rendered_content(),
//         }
//     }
// }