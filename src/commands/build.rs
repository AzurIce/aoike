use std::error::Error;
use std::fs;
use std::fs::{copy, create_dir_all, DirEntry, try_exists};
use std::path::{Path, PathBuf};
use pathdiff::diff_paths;
use pulldown_cmark::{html, Options, Parser};
use tera::{Context, Tera};
use crate::structures::{Post, PostData};

pub const POST_DIR: &str = "posts";
pub const SITE_DIR: &str = "site";
pub const THEMES_DIR: &str = "themes";
pub const THEME: &str = "aoike";

/// Perform a full site build
pub fn build(src_dir: &PathBuf) {
    let post_dir = src_dir.join(POST_DIR);
    let site_dir = src_dir.join(SITE_DIR);
    let theme_dir = src_dir.join(THEMES_DIR).join(THEME);
    println!("building with src_dir={:?}", src_dir);

    // Clean up
    if try_exists(&site_dir).unwrap_or(false) {
        fs::remove_dir_all(&site_dir).expect("Clean failed");
    }

    // Compile template
    let template_path = theme_dir.join("**/*.html");
    println!("Loading templates from {:?}", template_path);
    let mut tera = match Tera::new(template_path.to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing error(s): {}", e);
        }
    };
    tera.autoescape_on(vec![".html"]);

    // Copy theme files
    let theme_files = get_files(&theme_dir).expect("Get theme files failed");
    for entry in theme_files {
        let src_path = entry.path();
        let dst_path = site_dir.join(src_path.strip_prefix(&theme_dir).unwrap());

        let parent = dst_path.parent().expect("Get Parent failed");
        if !try_exists(parent).expect("Try exist failed") {
            create_dir_all(parent).expect("Create dir failed");
        }

        if entry.path().extension().map(|s| s == "html").unwrap_or(false) {
            continue;
        }
        copy(src_path, dst_path).expect("Copy theme file failed");
    }

    let mut post_data_vec: Vec<PostData> = Vec::new();

    // Build posts
    let files = get_files(&post_dir).expect("Get posts files failed");
    for entry in files {
        let src_path = entry.path();

        // println!("src_path: {:?}, post_dir: {:?}, rel_root_path: {:?}", src_path, post_dir, rel_root_path);
        let dst_path = site_dir.join(src_path.strip_prefix(&post_dir).unwrap());

        let parent = dst_path.parent().expect("Get Parent failed");
        if !try_exists(parent).expect("Try exist failed") {
            create_dir_all(parent).expect("Create dir failed");
        }

        if entry.path().extension().map(|s| s == "md").unwrap_or(false) {
            let post = Post::from_entry(entry);
            let dst_path = dst_path.with_extension("html");
            println!("Building Post: {:?} to {:?}", src_path, dst_path);

            let post_data = PostData::from_post(&post);
            let rel_root_path = calc_rel_path(&post_dir, &src_path.parent().unwrap());

            let mut context = Context::new();
            context.insert("post", &post_data);
            context.insert("rel_root_path", &rel_root_path);
            let output = tera.render("post.html", &context).expect("Failed to build post");
            fs::write(dst_path, output).expect("Failed to write html file");

            post_data_vec.push(post_data);
        } else {
            copy(src_path, dst_path).expect("Copy file failed");
        }
    }

    // Build index
    let mut context = Context::new();
    context.insert("posts", post_data_vec.as_slice());
    context.insert("rel_root_path", ".");
    let output = tera.render("main.html", &context).expect("Failed to build post");
    fs::write(site_dir.join("index.html"), output).expect("Failed to write html file");
}

fn get_files(dir: &PathBuf) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let mut files: Vec<DirEntry> = vec![];

    for entry in fs::read_dir(dir)? {
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

fn calc_rel_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> String {
    let rel_path = diff_paths(path, base).expect("Calc relative root path failed");
    let rel_path = rel_path.to_str().map(|s| {
        if s.len() == 0 {
            "."
        } else {
            s
        }
    }).unwrap().to_string();
    rel_path
}