fn main() {
    println!("cargo:rerun-if-changed=doc-src");

    // Parse markdown files to HTML using aoike-build
    let posts = aoike_dioxus::aoike::build::parse_posts("doc-src/posts");
    let index = aoike_dioxus::aoike::build::parse_post("doc-src/index.md");

    // Convert to Dioxus posts and generate RSX code
    let dioxus_posts: Vec<_> = posts.into_iter().map(aoike_dioxus::build::DioxusPost::from).collect();
    let dioxus_index = aoike_dioxus::build::DioxusPost::from(index);

    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = aoike_dioxus::build::generate_code(dioxus_posts, dioxus_index);
    std::fs::write(out_dir.join("docsgen.rs"), code).unwrap();
}
