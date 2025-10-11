use aoike_dioxus::aoike::build::{Entity, post::Post};

fn main() {
    println!("cargo:rerun-if-changed=doc-src");

    // Parse markdown files to HTML using aoike-build
    let posts = aoike_dioxus::aoike::build::parse_posts("doc-src/posts");
    let index = Entity::new("doc-src/index.md");
    let index = aoike_dioxus::build::DioxusPost::from(Post::try_from(index).unwrap());

    // Convert to Dioxus posts and generate RSX code
    let dioxus_posts: Vec<_> = posts
        .into_iter()
        .map(aoike_dioxus::build::DioxusPost::from)
        .collect();
    let dioxus_index = aoike_dioxus::build::DioxusPost::from(index);

    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = aoike_dioxus::build::generate_code(dioxus_posts, dioxus_index);
    std::fs::write(out_dir.join("docsgen.rs"), code).unwrap();
}
