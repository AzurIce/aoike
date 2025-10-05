use regex::Regex;

pub fn inject_str(source: &str, inject: &str, tag: &str, pos: Option<&str>) -> String {
    // 如果 <!-- tag_start --> <!-- tag_end --> 存在就替换，不存在就插入到查找 pos 的位置或最后
    let start_tag = format!("<!-- {}_START -->", tag.to_ascii_uppercase());
    let end_tag = format!("<!-- {}_END -->", tag.to_ascii_uppercase());

    let mut new_str = String::new();

    let start_idx = source.find(&start_tag).or(pos.and_then(|p| source.find(p))).unwrap_or(source.len());
    let end_idx = source
        .find(&end_tag)
        .map(|idx| idx + end_tag.len())
        .or(pos.and_then(|p| source.find(p))).unwrap_or(source.len());

    let before = source.get(..start_idx).unwrap_or("");
    let after = source.get(end_idx..).unwrap_or("");
    new_str.push_str(before);
    new_str.push_str(&[start_tag, inject.to_string(), end_tag].join("\n"));
    new_str.push_str(after);
    new_str
}

pub fn get_ref_paths(html: &str) -> Vec<String> {
    let re = Regex::new(r#"src="([^"]+)"#).unwrap();
    re.captures_iter(html)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .collect()
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
