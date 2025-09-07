use proc_macro2::TokenStream;
use quote::ToTokens;
use std::{path::Path, str::FromStr};

use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=blogs");
    let blogs = parse_blogs("blogs");
    let out_dir = std::env::current_dir().unwrap().join("src");

    let token = quote::quote! {
        use dioxus::prelude::*;

        pub const BLOGS: std::cell::LazyCell<Vec<aoike::BlogMeta>> = std::cell::LazyCell::new(|| {
            vec![#(#blogs),*]
        });
    };

    let code = prettyplease::unparse(&syn::parse_quote! {
        #token
    });
    std::fs::write(out_dir.join("docsgen.rs"), code).unwrap();
}

struct Blog {
    title: String,
    rsx: String,
}

impl ToTokens for Blog {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Blog { title, rsx } = self;
        let rsx = TokenStream::from_str(&rsx).unwrap();
        tokens.extend(quote::quote! {
            aoike::BlogMeta {
                title: #title.to_string(),
                content_fn: std::sync::Arc::new(|| rsx! { #rsx }),
            }
        });
    }
}

fn parse_blogs(dir: impl AsRef<Path>) -> Vec<Blog> {
    let dir = dir.as_ref();

    let mut blogs = Vec::new();
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
        let title = entry.file_name().to_string_lossy().to_string();
        let raw_doc = std::fs::read_to_string(entry.path()).unwrap();

        let parser = pulldown_cmark::Parser::new(&raw_doc);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        let dom = dioxus_rsx_rosetta::Dom::parse(&html).unwrap();
        let rsx = dioxus_rsx_rosetta::rsx_from_html(&dom);
        let rsx = dioxus_autofmt::write_block_out(&rsx).unwrap();

        blogs.push(Blog { title, rsx });
    }

    blogs
}
