pub mod utils;

use proc_macro2::TokenStream;
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};
use quote::ToTokens;
use relative_path::{PathExt, RelativePath};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Post {
    pub file_path: PathBuf,
    pub ref_paths: Vec<String>,
    pub slug: String,
    pub title: String,
    pub summary_html: String,
    pub content_html: String,
    pub created: i64,
    pub updated: i64,
}

pub fn parse_post(path: impl AsRef<Path>) -> Post {
    let path = path.as_ref();

    let filename = path
        .with_extension("")
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let raw_doc = std::fs::read_to_string(path).unwrap();

    let parser = pulldown_cmark::Parser::new(&raw_doc);
    let mut iterator = pulldown_cmark::TextMergeStream::new(parser);

    let mut heading_start = None;
    while let Some(e) = iterator.next() {
        if matches!(
            e,
            Event::Start(Tag::Heading {
                level: HeadingLevel::H1,
                ..
            })
        ) {
            heading_start = Some(e);
            break;
        }
    }
    let heading_html = heading_start.map(|_| {
        let mut html = String::new();
        pulldown_cmark::html::push_html(
            &mut html,
            iterator.take_while(|e| !matches!(e, Event::End(TagEnd::Heading(HeadingLevel::H1)))),
        );
        html
    });
    let title = heading_html.unwrap_or(filename.clone());
    let slug = slug::slugify(filename);

    let parser = pulldown_cmark::Parser::new(&raw_doc);
    let mut content_html = String::new();
    pulldown_cmark::html::push_html(&mut content_html, parser);

    // 提取 HTML 摘要(前200字符,保持标签完整)
    let filtered_html = utils::remove_html_tag(&content_html, &["h1"]);
    let summary_html = utils::extract_html_summary(&filtered_html, 200);

    let created = git_created_ts(path);
    let updated = git_updated_ts(path);
    Post {
        file_path: path.to_path_buf(),
        ref_paths: utils::get_ref_paths(&content_html),
        title,
        slug,
        summary_html,
        content_html,
        created,
        updated,
    }
}

pub fn parse_posts(dir: impl AsRef<Path>) -> Vec<Post> {
    let dir = dir.as_ref();

    let mut posts = Vec::new();
    for entry in WalkDir::new(dir) {
        let Ok(entry) = entry else {
            continue;
        };
        if entry.file_type().is_dir() {
            continue;
        }
        if entry.path().extension().map(|e| e != "md").unwrap_or(false) {
            continue;
        }
        let path = entry.path();

        posts.push(parse_post(path));
    }

    posts
}

pub fn git_updated_ts(path: &Path) -> i64 {
    use std::process::Command;
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%ct")
        .arg(path)
        .output();
    parse_git_ts(output)
}

pub fn git_created_ts(path: &Path) -> i64 {
    use std::process::Command;
    let output = Command::new("git")
        .arg("log")
        .arg("--diff-filter=A")
        .arg("-1")
        .arg("--format=%ct")
        .arg(path)
        .output();
    parse_git_ts(output)
}

fn parse_git_ts(output: std::io::Result<std::process::Output>) -> i64 {
    match output {
        Ok(out) if out.status.success() => {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            s.parse::<i64>().unwrap_or(0)
        }
        _ => 0,
    }
}

impl ToTokens for Post {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            slug,
            title,
            summary_html,
            content_html,
            created,
            updated,
            ..
        } = self;
        tokens.extend(quote::quote! {
            aoike::PostData {
                title: #title.to_string(),
                slug: #slug.to_string(),
                summary_html: #summary_html.to_string(),
                content_html: #content_html.to_string(),
                created: aoike::time::UtcDateTime::from_unix_timestamp(#created).unwrap(),
                updated: aoike::time::UtcDateTime::from_unix_timestamp(#updated).unwrap(),
            }
        });
    }
}

pub fn get_assets_trunk_data(
    posts: &Vec<Post>,
    index: &Post,
    root_dir: impl AsRef<Path>,
) -> String {
    posts
        .iter()
        .chain(std::iter::once(index))
        .flat_map(|p| {
            let file_path = Path::new(&p.file_path);
            p.ref_paths
                .iter()
                .filter_map(|p| RelativePath::from_path(p).ok())
                .map(|ref_path| {
                    let ref_path = ref_path.to_path(file_path.parent().unwrap());

                    let relative_path = ref_path.relative_to(&root_dir).unwrap();
                    let target_path = relative_path.to_path("");

                    let target_dir = target_path.parent().unwrap(); //.join(&p.slug);
                    format!(
                        r#"<link rel="copy-file" href="{}" data-target-path="{}" data-trunk>"#,
                        ref_path.to_string_lossy(),
                        target_dir.to_string_lossy()
                    )
                })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn generate_code(posts: Vec<Post>, index: Post) -> String {
    let token = quote::quote! {
        pub fn index() -> &'static aoike::PostData {
            static INDEX: std::sync::LazyLock<aoike::PostData> = std::sync::LazyLock::new(|| {
                #index
            });
            &INDEX
        }
        pub fn posts() -> &'static [aoike::PostData] {
            static POSTS: std::sync::LazyLock<Vec<aoike::PostData>> = std::sync::LazyLock::new(|| {
                let mut posts = vec![#(#posts),*];
                posts.sort_by(|a, b| b.created.cmp(&a.created));
                posts
            });
            &POSTS
        }
    };

    prettyplease::unparse(&syn::parse_quote! {
        #token
    })
}
