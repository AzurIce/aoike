use std::error::Error;
use std::fs;
use std::fs::{copy, create_dir_all, DirEntry, try_exists};
use std::path::{Path, PathBuf};
use minijinja::filters::safe;
use minijinja::{Environment, context, path_loader};
use pathdiff::diff_paths;
use pulldown_cmark::{Parser, Options};
// use tera::{Context, Tera};
use crate::structures::{Post, SourceFile};

pub const POST_DIR: &str = "posts";
pub const SITE_DIR: &str = "site";
pub const THEMES_DIR: &str = "themes";
pub const THEME: &str = "aoike";

/// Perform a full site build
pub fn build(src_dir: &PathBuf) {
    let post_dir = src_dir.join(POST_DIR);
    let site_dir = src_dir.join(SITE_DIR);
    let theme_dir = src_dir.join(THEMES_DIR).join(THEME);
    println!("[command/build]: building with src_dir={src_dir:?}");

    // Clean up
    println!("[command/build]: cleaning {site_dir:?}");
    if try_exists(&site_dir).unwrap_or(false) {
        fs::remove_dir_all(&site_dir).expect("Clean failed");
    }

    // // Compile template
    // let template_path = theme_dir.join("**/*.html");
    // println!("Loading templates from {:?}", template_path);
    // let mut tera = match Tera::new(template_path.to_str().unwrap()) {
    //     Ok(t) => t,
    //     Err(e) => {
    //         panic!("Parsing error(s): {}", e);
    //     }
    // };
    // tera.autoescape_on(vec![".html"]);

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
        // println!("[command/build]: copying theme file from {src_path:?} to {dst_path:?}");
        copy(src_path, dst_path).expect("Copy theme file failed");
    }

    // let mut post_data_vec: Vec<PostData> = Vec::new();

    // let src_files = vec![];
    // Build posts
    println!("[command/build]: getting source files from src_dir={src_dir:?}");
    let mut src_files: Vec<SourceFile> = Vec::new();
    let files = get_files(&post_dir).expect("Get posts files failed");
    let mut env = Environment::new();
    env.set_loader(path_loader(theme_dir.join("templates")));
    env.add_filter("safe", safe);
    // let post_template = fs::read_to_string(theme_dir.join("dist").join("templates").join("post.html")).expect("cannot read post template");
    // env.add_template("post", &post_template).expect("cannot add post template");

    for entry in files {
        let path = entry.path();
        // println!("{path:?}")

        let rel_path = calc_rel_path(&path, &src_dir);
        let rel_root_path = calc_rel_path(&src_dir, &path.parent().unwrap());
        let src_file = SourceFile::from(path);
        match &src_file {
            SourceFile::MarkdownFile(path) => {
                let dst_path = site_dir.join(rel_path).with_extension("html");
                // println!("[command/build]: building markdown file from {path:?} to {dst_path:?}");
                let post = Post::from_path(path);
                let post_template = env.get_template("post.html").expect("cannot get post template");
                // println!("{}", post.document);
                let rendered_post = post_template.render(context! {
                    rel_root_path,
                    post => context! {
                        title => post.title.as_str(),
                        draft => post.draft,
                        create_time => &post.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                        update_time => &post.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                        document => post.document.as_str()
                    }
                }).unwrap();

                let parent = dst_path.parent().expect("Get Parent failed");
                if !try_exists(parent).expect("Try exist failed") {
                    create_dir_all(parent).expect("Create dir failed");
                }
                fs::write(dst_path, rendered_post).expect("Write file failed")
            },
            SourceFile::OtherFile(path) => {
                let dst_path = site_dir.join(rel_path);
                // println!("[command/build]: copying file from {path:?} to {dst_path:?}");

                let parent = dst_path.parent().expect("Get Parent failed");
                if !try_exists(parent).expect("Try exist failed") {
                    create_dir_all(parent).expect("Create dir failed");
                }
                copy(path, dst_path).expect("Copy file failed");
            }
        };
        src_files.push(src_file);

        // println!("src_path: {:?}, post_dir: {:?}, rel_root_path: {:?}", src_path, post_dir, rel_root_path);
        // let dst_path = site_dir.join(path.strip_prefix(&post_dir).unwrap());

        // let parent = dst_path.parent().expect("Get Parent failed");
        // if !try_exists(parent).expect("Try exist failed") {
        //     create_dir_all(parent).expect("Create dir failed");
        // }

        // if entry.path().extension().map(|s| s == "md").unwrap_or(false) {
        //     let post = Post::from_entry(entry);
        //     let dst_path = dst_path.with_extension("html");
        //     println!("Building Post: {:?} to {:?}", path, dst_path);

        //     let post_data = PostData::from_post(&post);
        //     let rel_root_path = calc_rel_path(&post_dir, &src_path.parent().unwrap());

        //     let mut context = Context::new();
        //     context.insert("post", &post_data);
        //     context.insert("rel_root_path", &rel_root_path);
        //     let output = tera.render("post.html", &context).expect("Failed to build post");
        //     fs::write(dst_path, output).expect("Failed to write html file");

        //     post_data_vec.push(post_data);
        // } else {
        //     copy(src_path, dst_path).expect("Copy file failed");
        // }
    }

    // // Build index
    // let mut context = Context::new();
    // context.insert("posts", post_data_vec.as_slice());
    // context.insert("rel_root_path", ".");
    // let output = tera.render("main.html", &context).expect("Failed to build post");
    // fs::write(site_dir.join("index.html"), output).expect("Failed to write html file");
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
            if entry.file_name() == "node_modules" {
                continue;
            }
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