use std::error::Error;
use std::fs;
use std::fs::{copy, create_dir_all, DirEntry, try_exists};
use std::io::ErrorKind::NotFound;
use std::path::{Path, PathBuf};
use pulldown_cmark::{html, Options, Parser};

const POST_DIR: &str = "posts";
const SITE_DIR: &str = "site";

/// Perform a full site build
pub fn build(src_dir: &PathBuf) {
    let post_dir = src_dir.join(POST_DIR);
    let site_dir = src_dir.join(SITE_DIR);
    println!("building with src_dir={src_dir:?}");

    fs::remove_dir_all(&site_dir).expect("Clean failed");

    let files = get_files(&post_dir).expect("Get files failed");
    println!("{files:?}");
    for entry in files {
        let src_path = entry.path();
        // println!("{path:?}");
        let dst_path = site_dir.join(src_path.strip_prefix(&post_dir).unwrap());
        // println!("{path:?}");

        let parent = dst_path.parent().expect("Get Parent failed");
        if !try_exists(parent).expect("Try exist failed") {
            create_dir_all(parent).expect("Create dir failed");
        }

        if entry.path().extension().map(|s| s == "md").unwrap_or(false) {
            let dst_path = dst_path.with_extension("html");
            let markdown = fs::read_to_string(src_path).expect("Failed to read markdown file");
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_TASKLISTS);
            let parser = Parser::new_ext(&markdown, options);

            let mut html = String::new();
            html::push_html(&mut html, parser);

            fs::write(dst_path, html).expect("Failed to write html file");
            println!("{entry:?}")
        } else {
            copy(src_path, dst_path).expect("Copy file failed");
        }
    }
}

fn get_files(dir: &PathBuf) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let mut files: Vec<DirEntry> = vec![];

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;

        if entry.file_name().to_str().and_then(|s| s.chars().next())
            .map(|c| c == '_').unwrap_or(false){
            continue;
        }

        if entry.path().is_dir() {
            let mut inner_files = get_files(&entry.path())?;
            files.append(&mut inner_files);
        } else {
            files.push(entry);
        }

    }
    Ok(files)
}