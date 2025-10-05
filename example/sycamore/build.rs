fn main() {
    println!("cargo:rerun-if-changed=doc-src");

    // Parse markdown files to HTML using aoike-build
    let posts = aoike::build::parse_posts("doc-src/posts");
    let index = aoike::build::parse_post("doc-src/index.md");

    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = aoike::build::generate_code(posts, index);
    std::fs::write(out_dir.join("docsgen.rs"), code).unwrap();
}
