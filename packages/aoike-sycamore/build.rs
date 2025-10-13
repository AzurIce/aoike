use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::Path,
};

use anyhow::Context;
use sha1::{Digest, Sha1};
use zip::write::SimpleFileOptions;

fn main() {
    println!("cargo:rerun-if-changed=css");
    println!("cargo:rerun-if-changed=css/elements/_article.scss");
    println!("cargo:rerun-if-changed=css/_var.scss");
    println!("cargo:rerun-if-changed=css/main.scss");
    println!("cargo:rerun-if-changed=css/uno.scss");

    // Calculate SHA1 hash of css directory
    let sha1_hash = calculate_dir_sha1("css").expect("failed to calculate css directory sha1");
    println!("CSS directory SHA1: {}", sha1_hash);

    zip_dir(
        &mut walkdir::WalkDir::new("css")
            .into_iter()
            .filter_map(|e| e.ok()),
        &Path::new("css"),
        File::create("css.zip").expect("failed to create css.zip"),
        zip::CompressionMethod::Deflated,
        &sha1_hash,
    )
    .expect("failed to zip css/");
}

fn calculate_dir_sha1(dir: &str) -> Result<String, anyhow::Error> {
    let mut hasher = Sha1::new();
    let mut entries: Vec<_> = walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    // Sort to ensure consistent hash
    entries.sort_by_key(|e| e.path().to_path_buf());

    for entry in entries {
        let path = entry.path();
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Hash file path and content
        hasher.update(path.to_string_lossy().as_bytes());
        hasher.update(&buffer);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &Path,
    writer: T,
    method: zip::CompressionMethod,
    sha1_hash: &str,
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

    // Add SHA1 file at the end
    println!("adding sha1 file...");
    zip.start_file("sha1", options)?;
    zip.write_all(sha1_hash.as_bytes())?;

    zip.finish()?;
    Ok(())
}
