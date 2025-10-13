use std::path::Path;

use regex::Regex;
use time::UtcDateTime;

pub fn patch_file(
    path: impl AsRef<Path>,
    inject: &str,
    tag: &str,
    pos: Option<&str>,
) -> Result<(), anyhow::Error> {
    let path = path.as_ref();

    let source = std::fs::read_to_string(path)?;
    let (res, old_inject) = inject_str(&source, inject, tag, pos);
    if let Some(old_inject) = old_inject.as_deref()
        && old_inject.trim() != inject.trim()
    {
        println!("cargo:warning=patching file {path:?}");
        std::fs::write(path, res)?;
    }
    Ok(())
}

/// Inject string to source with tag and pos
///
/// Returns: (new_str, old_inject)
pub fn inject_str(
    source: &str,
    inject: &str,
    tag: &str,
    pos: Option<&str>,
) -> (String, Option<String>) {
    // 使用正则表达式来查找和替换指定的注释块
    let start_tag = format!("<!-- {}_START -->", tag.to_ascii_uppercase());
    let end_tag = format!("<!-- {}_END -->", tag.to_ascii_uppercase());

    let re = Regex::new(&format!(
        "(?s){}(.*?){}",
        regex::escape(&start_tag),
        regex::escape(&end_tag)
    ))
    .unwrap();

    if let Some(caps) = re.captures(source) {
        let old_inject = caps.get(1).map(|m| m.as_str().to_string());
        let out = re
            .replace(source, format!("{}\n{}\n{}", &start_tag, inject, &end_tag))
            .to_string();
        (out, old_inject)
    } else {
        // 没有注释块，插入到 pos 对应的位置
        let insert_pos = pos.and_then(|p| source.find(p)).unwrap_or(source.len());
        let (before, after) = source.split_at(insert_pos);
        let mut out = String::new();
        out.push_str(before);
        out.push_str(&format!("{}\n{}\n{}", &start_tag, inject, &end_tag));
        out.push_str(after);
        (out, None)
    }
}

pub fn get_ref_paths(html: &str) -> Vec<String> {
    // 排除 data:image 开头的内联数据
    // Rust's regex does not support lookahead/lookbehind, so match all src="..." and filter in code.
    let re = Regex::new(r#"src="([^"]+)""#).unwrap();
    re.captures_iter(html)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .filter(|s| !s.starts_with("data:"))
        .collect()
}

pub fn get_tag_content(html: &str, tag: &str) -> Option<String> {
    let re = Regex::new(&format!("<{tag}>(?s:(.*?))</{tag}>")).unwrap();
    let cap = re.captures_iter(html).next();
    cap.map(|cap| cap.get(1).unwrap().as_str().trim().to_string())
}

pub fn get_html_h1(html: &str) -> Option<String> {
    get_tag_content(html, "h1")
}

pub fn remove_html_tag(html: &str, tags: &[&str]) -> String {
    // tags: ["h1", "h2"] etc.
    if tags.is_empty() {
        return html.to_string();
    }

    // 构建标准正则表达式（不使用反向引用，因为 Rust 的标准 regex 不支持反向引用）
    // 针对每一个 tag 单独生成正则，并分别替换
    let mut result = html.to_string();

    for &tag in tags {
        let tag_escaped = regex::escape(tag);

        // 匹配形如 <tag ...>...</tag> （非嵌套）、以及自闭合 <tag ... />
        // 非贪婪匹配内容，适合移除简单的 h1/h2 结构
        let open_close_pat = format!(r"(?is)<{tag}\b[^>]*>.*?</{tag}\s*>", tag = tag_escaped);
        let self_close_pat = format!(r"(?is)<{tag}\b[^>]*/\s*>", tag = tag_escaped);

        let open_close_re = regex::RegexBuilder::new(&open_close_pat)
            .case_insensitive(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        let self_close_re = regex::RegexBuilder::new(&self_close_pat)
            .case_insensitive(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        result = open_close_re.replace_all(&result, "").to_string();
        result = self_close_re.replace_all(&result, "").to_string();
    }

    result.trim().to_string()
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

    out.trim().to_string()
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

pub fn git_updated_datetime(path: &Path) -> UtcDateTime {
    UtcDateTime::from_unix_timestamp(git_updated_ts(path)).unwrap()
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

pub fn git_created_datetime(path: &Path) -> UtcDateTime {
    UtcDateTime::from_unix_timestamp(git_created_ts(path)).unwrap()
}
