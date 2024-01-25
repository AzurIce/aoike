use axum::response::sse;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use std::{fs, path::PathBuf};

#[cfg(test)]
mod test {
    use syntect::{
        html::{ClassStyle, ClassedHTMLGenerator},
        parsing::SyntaxSet,
        util::LinesWithEndings,
    };

    #[test]
    fn test_syntect() {
        let ss = SyntaxSet::load_defaults_newlines();

        // Rust
        let code_rs = "// Rust source
fn main() {
    println!(\"Hello World!\");
}";

        let sr_rs = ss.find_syntax_by_extension("rs").unwrap();
        let mut rs_html_generator =
            ClassedHTMLGenerator::new_with_class_style(sr_rs, &ss, ClassStyle::Spaced);
        for line in LinesWithEndings::from(code_rs) {
            rs_html_generator
                .parse_html_for_line_which_includes_newline(line)
                .unwrap();
        }
        let html_rs = rs_html_generator.finalize();
        println!("{html_rs}");

        // writeln!(html, "<pre class=\"code\">")?;
        // writeln!(html, "{}", html_rs)?;
        // writeln!(html, "</pre>")?;
    }
}

pub enum SourceFile {
    MarkdownFile(PathBuf),
    OtherFile(PathBuf),
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
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, ThemeSet},
    html::{
        append_highlighted_html_for_styled_line, highlighted_html_for_string,
        start_highlighted_html_snippet, IncludeBackground,
    },
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

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
            create_time: DateTime::from_utc(
                NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                Local::now().offset().clone(),
            ),
            update_time: DateTime::from_utc(
                NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
                Local::now().offset().clone(),
            ),
            document: String::new(),
        }
    }
}

fn extract_frontmatter(content: String) -> (Option<String>, String) {
    if content.starts_with("---\n") {
        if let Some(fm_end) = content[4..].find("---\n") {
            let yaml = content[4..fm_end + 4].to_string();
            let content = content[fm_end + 2 * 4..].to_string();
            return (Some(yaml), content);
        }
    }
    (None, content)
}

impl Post {
    pub fn from_path(path: &PathBuf) -> Self {
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let title = filename.clone();
        let document = fs::read_to_string(path)
            .and_then(|content| {
                let content = content.replace("\r\n", "\n");
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_FOOTNOTES);
                options.insert(Options::ENABLE_TABLES);
                options.insert(Options::ENABLE_TASKLISTS);

                let (frontmatter, content) = extract_frontmatter(content);

                let parser = Parser::new_ext(&content, options);
                // https://github.com/raphlinus/pulldown-cmark/issues/167
                let ss = SyntaxSet::load_defaults_newlines();
                let ts = ThemeSet::load_defaults();
                // println!("{:?}", ts.themes.keys());
                let theme = &ts.themes["base16-ocean.light"];
                let mut to_highlight = String::new();
                let mut in_code_block = false;
                let mut new_p = Vec::new();
                for event in parser {
                    match event {
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                            in_code_block = true
                        }
                        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                            if in_code_block {
                                let syntax = ss
                                    .find_syntax_by_token(&lang)
                                    .unwrap_or(ss.find_syntax_plain_text());
                                let mut highlighter = HighlightLines::new(syntax, theme);
                                let bg = theme.settings.background.unwrap_or(Color::WHITE);
                                let mut output = format!("<pre style=\"background-color:#{:02x}{:02x}{:02x};\" class=\"language_{lang}\">\n",bg.r, bg.g, bg.b);
                                // let (mut output, bg) = start_highlighted_html_snippet(theme);

                                for line in LinesWithEndings::from(&to_highlight) {
                                    output.push_str("<span class=\"line\">");
                                    let regions = highlighter
                                        .highlight_line(line, &ss)
                                        .expect("failed to highlight");
                                    append_highlighted_html_for_styled_line(
                                        &regions[..],
                                        IncludeBackground::IfDifferent(bg),
                                        &mut output,
                                    )
                                    .expect("failed to append highlighted html");
                                    output.push_str("</span>");
                                }
                                output.push_str("</pre>\n");

                                new_p.push(Event::Html(CowStr::Boxed(output.into())));
                                to_highlight.clear();
                            }
                        }
                        Event::Text(t) => {
                            if in_code_block {
                                to_highlight.push_str(&t);
                            } else {
                                new_p.push(Event::Text(t))
                            }
                        }
                        e => new_p.push(e),
                    }
                }

                let mut html = String::new();
                html::push_html(&mut html, new_p.into_iter());
                Ok(html)
            })
            .unwrap();

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
