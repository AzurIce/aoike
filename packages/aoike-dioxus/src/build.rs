use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;

pub fn html_to_rsx(html: &str) -> String {
    let dom = dioxus_rsx_rosetta::Dom::parse(html).unwrap();
    let rsx = dioxus_rsx_rosetta::rsx_from_html(&dom);
    let rsx = dioxus_autofmt::write_block_out(&rsx).unwrap();
    rsx
}

pub struct DioxusPost {
    pub slug: String,
    pub title: String,
    pub summary_html: String,
    pub content_html: String,
    pub created: i64,
    pub updated: i64,
}

impl From<aoike::build::Post> for DioxusPost {
    fn from(post: aoike::build::Post) -> Self {
        Self {
            slug: post.slug,
            title: post.title,
            summary_html: post.summary_html,
            content_html: post.content_html,
            created: post.created,
            updated: post.updated,
        }
    }
}

impl ToTokens for DioxusPost {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            slug,
            title,
            summary_html,
            content_html,
            created,
            updated,
        } = self;
        let summary_rsx = TokenStream::from_str(&html_to_rsx(&summary_html)).unwrap();
        let content_rsx = TokenStream::from_str(&html_to_rsx(&content_html)).unwrap();
        tokens.extend(quote::quote! {
            aoike_dioxus::PostData {
                title: #title.to_string(),
                slug: #slug.to_string(),
                summary_rsx: aoike_dioxus::RsxFn::new(|| rsx! { #summary_rsx }),
                content_rsx: aoike_dioxus::RsxFn::new(|| rsx! { #content_rsx }),
                created: aoike_dioxus::time::UtcDateTime::from_unix_timestamp(#created).unwrap(),
                updated: aoike_dioxus::time::UtcDateTime::from_unix_timestamp(#updated).unwrap(),
            }
        });
    }
}

pub fn generate_code(posts: Vec<DioxusPost>, index: DioxusPost) -> String {
    let token = quote::quote! {
        use dioxus::prelude::*;

        pub fn index() -> &'static aoike_dioxus::PostData {
            static INDEX: std::sync::LazyLock<aoike_dioxus::PostData> = std::sync::LazyLock::new(|| {
                #index
            });
            &INDEX
        }
        pub fn posts() -> &'static [aoike_dioxus::PostData] {
            static POSTS: std::sync::LazyLock<Vec<aoike_dioxus::PostData>> = std::sync::LazyLock::new(|| {
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
