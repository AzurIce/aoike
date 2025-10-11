pub mod post;
pub mod utils;

use proc_macro2::TokenStream;
use quote::ToTokens;
use relative_path::{PathExt, RelativePath};
use std::path::{Path, PathBuf};
use time::UtcDateTime;

use walkdir::WalkDir;

use crate::build::post::Post;

#[derive(Debug, Clone)]
pub struct Entity {
    pub path: PathBuf,
    pub created: UtcDateTime,
    pub updated: UtcDateTime,
    pub content: Vec<u8>,
}

impl Entity {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let content = std::fs::read(path).unwrap();
        let created = utils::git_created_datetime(path);
        let updated = utils::git_updated_datetime(path);
        Self {
            path: path.to_path_buf(),
            created,
            updated,
            content,
        }
    }
    pub fn extension(&self) -> String {
        self.path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
    }
    pub fn base_name(&self) -> String {
        self.path
            .with_extension("")
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap()
            .to_string()
    }
    pub fn slug(&self) -> String {
        slug::slugify(self.path.file_name().and_then(|s| s.to_str()).unwrap())
    }
}

pub trait Parser {
    type Output;
    fn try_parse(entity: Entity) -> Result<Self::Output, anyhow::Error>;
}

pub fn parse_posts(dir: impl AsRef<Path>) -> Vec<Post> {
    let dir = dir.as_ref();

    let mut posts = Vec::new();
    for entry in WalkDir::new(dir)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
    {
        let entity = Entity::new(entry.path());
        println!("cargo:warning=entity: {}", entity.base_name());
        if let Ok(post) = Post::try_from(entity) {
            posts.push(post);
        }
    }

    posts
}

impl ToTokens for Post {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            title,
            summary_html,
            content_html,
            ..
        } = self;
        let slug = self.entity.slug();
        let created = self.entity.created.unix_timestamp();
        let updated = self.entity.updated.unix_timestamp();
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
            let file_path = Path::new(&p.path);
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
                let mut posts: Vec<aoike::PostData> = vec![#(#posts),*];
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
