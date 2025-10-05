use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
};

use anyhow::Context;
use zip::write::SimpleFileOptions;

fn main() {
    println!("cargo:rerun-if-changed=css");
    println!("cargo:rerun-if-changed=css/elements/_article.scss");
    println!("cargo:rerun-if-changed=css/_var.scss");
    println!("cargo:rerun-if-changed=css/main.scss");
    println!("cargo:rerun-if-changed=css/uno.scss");

    zip_dir(
        &mut walkdir::WalkDir::new("css")
            .into_iter()
            .filter_map(|e| e.ok()),
        &Path::new("css"),
        File::create("css.zip").expect("failed to create css.zip"),
        zip::CompressionMethod::Deflated,
    )
    .expect("failed to zip css/");
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
) -> Result<(), anyhow::Error>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let prefix = Path::new(prefix);
    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_string = name
            .to_str()
            .map(str::to_owned)
            .with_context(|| format!("{name:?} Is a Non UTF-8 Path"))?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            zip.start_file(path_as_string, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path_as_string:?} as {name:?} ...");
            zip.add_directory(path_as_string, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}
