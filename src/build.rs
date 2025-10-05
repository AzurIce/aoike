use proc_macro2::TokenStream;
use pulldown_cmark::{Event, HeadingLevel, Tag, TagEnd};
use quote::ToTokens;
use std::path::Path;

use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Post {
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
    let filtered_html = remove_html_tag(&content_html, &["h1"]);
    let summary_html = extract_html_summary(&filtered_html, 200);

    let created = git_created_ts(path);
    let updated = git_updated_ts(path);
    Post {
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

pub fn remove_html_tag(html: &str, tags: &[&str]) -> String {
    let mut out = String::with_capacity(html.len());
    let mut chars = html.char_indices().peekable();
    let mut skip_depth = 0; // 0: 正常输出,>0: 正在跳过某段 h1 内容

    while let Some((_, ch)) = chars.next() {
        if ch == '<' {
            // 解析标签名
            let mut tag_name = String::new();
            let mut is_close = false;

            if let Some(&(_, '/')) = chars.peek() {
                is_close = true;
                chars.next();
            }

            // 读标签名
            while let Some(&(_, c)) = chars.peek() {
                if c != '/' && c != '>' {
                    tag_name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }

            // 跳过到 '>'
            while let Some(&(_, c)) = chars.peek() {
                chars.next();
                if c == '>' {
                    break;
                }
            }

            // 判断是否为 h1
            if tags.iter().any(|t| tag_name.eq_ignore_ascii_case(t)) {
                if is_close {
                    if skip_depth > 0 {
                        skip_depth -= 1;
                    }
                } else {
                    skip_depth += 1;
                }
                continue; // 不输出 <h1> 或 </h1>
            }

            // 如果在 h1 内部,整体丢弃
            if skip_depth > 0 {
                continue;
            }

            // 还原标签到输出
            out.push('<');
            if is_close {
                out.push('/');
            }
            out.push_str(&tag_name);
            out.push('>');
        } else {
            if skip_depth == 0 {
                out.push(ch);
            }
        }
    }
    out
}

/// 从 HTML 字符串中提取前 `max_text_len` 个字符的摘要,不破坏标签结构
pub fn extract_html_summary(html: &str, max_text_len: usize) -> String {
    let mut out = String::new();
    let mut text_len = 0;
    let mut tag_stack: Vec<&str> = Vec::new();
    let mut chars = html.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if ch == '<' {
            // 解析标签
            let start = i;
            let mut tag_name = String::new();
            let mut is_close = false;
            let mut self_closing = false;

            // 跳过 '<'
            if let Some(&(_, '/')) = chars.peek() {
                is_close = true;
                chars.next();
            }

            // 提取标签名
            while let Some(&(_, c)) = chars.peek() {
                if c.is_ascii_alphabetic() || c == '-' {
                    tag_name.push(c);
                    chars.next();
                } else {
                    break;
                }
            }

            // 跳过属性部分
            while let Some(&(_, c)) = chars.peek() {
                if c == '>' {
                    chars.next();
                    break;
                } else if c == '/' {
                    chars.next();
                    if let Some(&(_, '>')) = chars.peek() {
                        self_closing = true;
                        chars.next();
                        break;
                    }
                } else {
                    chars.next();
                }
            }

            let tag_slice = &html[start..chars.peek().map(|(j, _)| *j).unwrap_or(html.len())];
            out.push_str(tag_slice);

            if !self_closing && !tag_name.is_empty() {
                if is_close {
                    tag_stack.pop();
                } else {
                    tag_stack.push(Box::leak(tag_name.into_boxed_str()));
                }
            }
        } else {
            // 文本内容
            if text_len < max_text_len {
                out.push(ch);
                if !ch.is_whitespace() {
                    text_len += 1;
                }
            } else {
                // 补全未关闭的标签
                out.extend(std::iter::repeat('.').take(3));
                for tag in tag_stack.into_iter().rev() {
                    out.push_str(&format!("</{}>", tag));
                }
                break;
            }
        }
    }

    out
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


