use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};
use std::{env, path::Path};
use walkdir::WalkDir;
// 使用简单文本处理，不需要 pulldown-cmark

#[proc_macro]
pub fn site(input: TokenStream) -> TokenStream {
    // let root_path = env!("CARGO_MANIFEST_DIR");
    let root_path = env::current_dir().unwrap();
    let site_dir = parse_macro_input!(input as LitStr).value();
    let site_dir = root_path.join(site_dir);
    let blogs_path = Path::new(&site_dir).join("blogs");
    
    let mut blogs = Vec::new();
    
    if blogs_path.exists() && blogs_path.is_dir() {
        for entry in WalkDir::new(&blogs_path) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "md") {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        let (title, document) = extract_blog_info(&content);
                        let _file_name = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                            .to_string();
                        
                        blogs.push(quote! {
                            aoike::BlogMeta {
                                title: #title.to_string(),
                                document: #document.to_string(),
                            }
                        });
                    }
                }
            }
        }
    }
    
    let site_dir = site_dir.to_string_lossy().to_string();
    let expanded = quote! {
        aoike::Site {
            root_dir: std::path::PathBuf::from(#site_dir),
            blogs: vec![#(#blogs),*],
        }
    };
    
    TokenStream::from(expanded)
}

fn extract_blog_info(content: &str) -> (String, String) {
    let mut title = String::from("Untitled");
    
    // 查找第一个 # 开头的行作为标题
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            title = trimmed[2..].trim().to_string();
            break;
        }
    }
    
    // 获取前200字符作为摘要
    let plain_text: String = content
        .lines()
        .skip_while(|line| line.trim().is_empty() || line.trim().starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .filter(|c| !c.is_ascii_control())
        .take(200)
        .collect();
    
    let document = if plain_text.len() > 197 {
        format!("{}...", plain_text)
    } else {
        plain_text
    };
    
    (title, document)
}