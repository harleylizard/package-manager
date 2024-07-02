use std::{fs, io};
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use crate::hash::Validator;
use crate::manifest::Platform;

mod manifest;
mod device;
mod download;
mod hash;

fn pause() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("TODO: panic message");
}

async fn run(platform: &Platform) {
    println!("Starting JVM...");
    let mut command = platform.get_command();

    let output = command.output().unwrap();
    println!("{}", String::from_utf8_lossy(&*output.stderr));
    println!("{}", String::from_utf8_lossy(&*output.stdout));
    println!("{}", output.status.success());

    println!("Enter input to exit console");
    pause();
}

async fn download() -> io::Result<()> {
    let read = &*fs::read_to_string("manifest.json")?;
    let manifest: manifest::Manifest = serde_json::from_str(read)?;

    let mut validator: Validator = Validator::get_or_create("checksum.json")?;

    let platform = manifest.get_platform().expect("Unsupported device.");
    for dependency in &platform.dependencies {
        if !&validator.compare(&dependency.name).await {
            let name = (&dependency.name).to_string();
            println!("Downloading {}", name);
            let certificate = download::download_dependency(dependency, &platform).await.expect("Failed to download dependency");

            validator.add(name, certificate);
        }
    }

    let path = platform.get_java_folder();
    if !Path::new(path).exists() {
        println!("Downloading {}", path);
        download::download_jre(&platform).await.expect("Failed to download jre");
    }

    let file = fs::File::create("checksum.json")?;
    serde_json::to_writer_pretty(file, &validator).expect("");

    run(&platform).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    download().await.unwrap();
}
