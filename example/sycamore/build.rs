use aoike::build::utils::inject_str;

fn main() {
    println!("cargo:rerun-if-changed=doc-src");

    aoike_sycamore::build::init_aoike_sycamore();

    // Parse markdown files to HTML using aoike-build
    let posts = aoike::build::parse_posts("doc-src/posts");
    let index = aoike::build::parse_post("doc-src/index.md");

    let assets = aoike::build::get_assets_trunk_data(&posts, &index, "doc-src");
    let index_html = std::fs::read_to_string("index.html").unwrap();
    std::fs::write(
        "index.html",
        inject_str(&index_html, &assets, "AOIKE_SYCAMORE_SITE_ASSETS", Some("</head>")),
    )
    .unwrap();
    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = aoike::build::generate_code(posts, index);
    std::fs::write(out_dir.join("docsgen.rs"), code).unwrap();
}
