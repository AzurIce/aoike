use std::{io::Cursor, path::Path};

use aoike::build::utils::inject_str;

// const CSS_ASSETS: Dir<'_> = include_dir::include_dir!("packages/aoike-sycamore/css");
const CSS_ARCHIVE: &[u8] = include_bytes!("../css.zip");

/// This does two things:
/// 1. export css assets to `assets/css`
/// 2. insert `<link>` tag into `index.html`
pub fn init_aoike_sycamore() {
    if !Path::new("assets/css").exists() {
        std::fs::create_dir_all("static/css").expect("failed to create statics/css");

        let cursor = Cursor::new(CSS_ARCHIVE);
        let mut zip = zip::ZipArchive::new(cursor).expect("failed to create zip archive");
        zip.extract("static/css").expect("failed to extract css assets into statics/css");
    }

    let index_html = std::fs::read_to_string("index.html").unwrap();
    let insert_str = r#"<link rel="css" href="static/css/uno.css" data-trunk>
<link rel="scss" href="static/css/main.scss" data-trunk>"#;

    std::fs::write(
        "index.html",
        inject_str(&index_html, insert_str, "AOIKE_SYCAMORE", Some("</head>")),
    )
    .unwrap();
}
