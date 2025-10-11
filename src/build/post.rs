use std::{ops::Deref, path::Path};

use anyhow::Context;

use crate::build::{utils, Entity, Parser};

#[derive(Debug, Clone)]
pub struct Post {
    pub entity: Entity,
    pub ref_paths: Vec<String>,
    pub title: String,
    pub summary_html: String,
    pub content_html: String,
}

impl Deref for Post {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl Post {
    pub fn from_html_entity(content_html: String, entity: Entity) -> Self {
        let title =
            utils::get_tag_content(&content_html, "h1").unwrap_or(entity.base_name().clone());
        let filtered_html = utils::remove_html_tag(&content_html, &["h1"]);
        let summary_html = utils::extract_html_summary(&filtered_html, 200);

        Self {
            entity,
            ref_paths: utils::get_ref_paths(&content_html),
            title,
            summary_html,
            content_html,
        }
    }
}

impl TryFrom<Entity> for Post {
    type Error = anyhow::Error;
    fn try_from(entity: Entity) -> Result<Self, Self::Error> {
        match entity.extension().as_str() {
            "md" => MarkdownPostParser::try_parse(entity),
            "typ" => TypstPostParser::try_parse(entity),
            _ => anyhow::bail!("unsupported file extension: {}", entity.extension()),
        }
    }
}

pub struct TypstPostParser;

impl Parser for TypstPostParser {
    type Output = Post;
    fn try_parse(entity: Entity) -> Result<Self::Output, anyhow::Error> {
        let content_html = compile_typst_to_html(&entity.path)?;
        let content_html = utils::get_tag_content(&content_html, "body").expect("no body");
        let content_html = utils::remove_html_tag(&content_html, &["h1"]);
        Ok(Post::from_html_entity(content_html, entity))
    }
}

fn compile_typst_to_html(path: impl AsRef<Path>) -> Result<String, anyhow::Error> {
    let child = std::process::Command::new("typst")
        .arg("compile")
        .arg(path.as_ref())
        .arg("-")
        .arg("-fhtml")
        .args(["--features", "html"])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .context("failed to spawn typst")?;

    let output = child
        .wait_with_output()
        .context("failed to wait for typst")?
        .stdout;
    String::from_utf8(output).context("contains invalid utf-8 content")
}

pub struct MarkdownPostParser;

impl Parser for MarkdownPostParser {
    type Output = Post;
    fn try_parse(entity: Entity) -> Result<Self::Output, anyhow::Error> {
        let content = std::str::from_utf8(&entity.content)?;

        let parser = pulldown_cmark::Parser::new(&content);
        let mut content_html = String::new();
        pulldown_cmark::html::push_html(&mut content_html, parser);

        Ok(Post::from_html_entity(content_html, entity))
    }
}

#[cfg(test)]
#[test]
fn test_compile_typst_to_html_basic() {
    let _result = compile_typst_to_html("example/sycamore/doc-src/posts/typst.typ").unwrap();
    let _result = utils::get_tag_content(&_result, "body").unwrap();
    let _result = utils::remove_html_tag(&_result, &["h1"]);

    // println!("{}", _result)
}
