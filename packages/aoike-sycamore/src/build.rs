use std::{io::Cursor, path::Path};

use aoike::build::utils::patch_file;

// const CSS_ASSETS: Dir<'_> = include_dir::include_dir!("packages/aoike-sycamore/css");
const CSS_ARCHIVE: &[u8] = include_bytes!("../css.zip");

/// This does two things:
/// 1. export css assets to `assets/css`
/// 2. insert `<link>` tag into `index.html`
pub fn init_aoike_sycamore() {
    let css_dir = Path::new("static/css");
    let sha1_file = css_dir.join("sha1");
    let needs_extraction = if css_dir.exists() && sha1_file.exists() {
        // Read existing sha1
        let existing_sha1 = std::fs::read_to_string(&sha1_file).ok();

        // Read sha1 from zip archive
        let cursor = Cursor::new(CSS_ARCHIVE);
        let mut zip = zip::ZipArchive::new(cursor).expect("failed to create zip archive");
        let archive_sha1 = zip.by_name("sha1").ok().and_then(|mut file| {
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content).ok()?;
            Some(content)
        });

        // Compare sha1 hashes
        match (existing_sha1, archive_sha1) {
            (Some(existing), Some(archive)) => {
                if existing.trim() == archive.trim() {
                    println!("cargo:warning=CSS assets are up to date (sha1: {})", existing.trim());
                    false
                } else {
                    println!("cargo:warning=CSS assets sha1 mismatch, re-extracting...");
                    println!("cargo:warning=  existing: {}", existing.trim());
                    println!("cargo:warning=  archive:  {}", archive.trim());
                    true
                }
            }
            _ => {
                println!("cargo:warning=SHA1 comparison failed, re-extracting CSS assets...");
                true
            }
        }
    } else {
        println!("cargo:warning=CSS assets directory not found, extracting...");
        true
    };

    if needs_extraction {
        std::fs::create_dir_all("static/css").expect("failed to create statics/css");

        let cursor = Cursor::new(CSS_ARCHIVE);
        let mut zip = zip::ZipArchive::new(cursor).expect("failed to create zip archive");
        zip.extract("static/css")
            .expect("failed to extract css assets into statics/css");
        println!("CSS assets extracted successfully");
    }

    let insert_str = r#"<link rel="css" href="static/css/uno.css" data-trunk>
<link rel="scss" href="static/css/main.scss" data-trunk>"#;
    patch_file("index.html", insert_str, "AOIKE_SYCAMORE", Some("</head>")).unwrap();
}
