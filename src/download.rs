use std::{fs, io};
use std::io::{Read, Write};

use bytes::Bytes;
use reqwest::Url;
use tempfile::NamedTempFile;
use zip::ZipArchive;

use crate::hash::*;
use crate::manifest::{Dependency, Platform};

#[inline]
async fn download_bytes(url: Url) -> Result<Bytes, reqwest::Error> {
    reqwest::get(url).await?.bytes().await
}

pub(crate) async fn download_dependency(dependency: &Dependency, platform: &Platform) -> io::Result<Certificate> {
    let bytes = download_bytes(dependency.get_url().expect("Failed to parse URL")).await.expect("Failed to download from URL");
    let path = &dependency.get_path(platform);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?
        }
    }
    fs::write(&path, bytes)?;

    let hash = hash_file(&path).await.expect("Failed to hash");
    Ok(Certificate::new(hash, path))
}

pub(crate) async fn download_jre(platform: &Platform) -> io::Result<Certificate> {
    let path = &platform.get_java_path();
    if !&path.exists() {
        fs::create_dir_all(&path)?
    }

    let mut file = NamedTempFile::new()?;
    let bytes = download_bytes(platform.get_java_url().expect("Failed to parse URL")).await.expect("Failed to download from URL");

    file.write_all(&*bytes)?;
    ZipArchive::new(&file)?.extract(&path.parent().unwrap())?;

    Ok(Certificate::new("placeholder".to_string(), &path))
}