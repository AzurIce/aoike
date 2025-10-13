use aoike::build::{post::Post, utils::patch_file, Entity};

fn main() {
    println!("cargo:rerun-if-changed=doc-src");

    aoike_sycamore::build::init_aoike_sycamore();

    // Parse markdown files to HTML using aoike-build
    let posts = aoike::build::parse_posts("doc-src/posts");
    let index = Entity::new("doc-src/index.md");
    let index = Post::try_from(index).unwrap();

    let assets = aoike::build::get_assets_trunk_data(&posts, &index, "doc-src");
    patch_file(
        "index.html",
        &assets,
        "AOIKE_SYCAMORE_SITE_ASSETS",
        Some("</head>"),
    )
    .unwrap();
    let out_dir = std::env::current_dir().unwrap().join("src");
    let code = std::fs::read_to_string(out_dir.join("docsgen.rs")).unwrap_or(String::new());
    let gen_code = aoike::build::generate_code(posts, index);
    if code != gen_code {
        std::fs::write(out_dir.join("docsgen.rs"), gen_code).unwrap();
    }
}
