use std::path::Path;

use aoike::build::utils::inject_str;
use include_dir::Dir;

const CSS_ASSETS: Dir<'_> = include_dir::include_dir!("packages/aoike-sycamore/css");

/// This does two things:
/// 1. export css assets to `assets/css`
/// 2. insert `<link>` tag into `index.html`
pub fn init_aoike_sycamore() {
    if !Path::new("assets/css").exists() {
        std::fs::create_dir_all("assets/css").expect("failed to create assets/css");
        CSS_ASSETS
            .extract("assets/css")
            .expect("failed to extract css assets into assets/css");
    }

    let index_html = std::fs::read_to_string("index.html").unwrap();
    let insert_str = r#"<link rel="css" href="assets/css/uno.css" data-trunk>
<link rel="scss" href="assets/css/main.scss" data-trunk>"#;

    std::fs::write(
        "index.html",
        inject_str(&index_html, insert_str, "AOIKE_SYCAMORE", Some("</head>")),
    )
    .unwrap();
}
