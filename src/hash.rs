use std::{fs, io};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize)]
#[derive(Deserialize)]
pub(crate) struct Validator {
    certificates: HashMap<String, Certificate>
}

impl Validator {
    pub(crate) fn get_or_create<P: AsRef<Path>>(path: P) -> io::Result<Validator> {
        if path.as_ref().exists() {
            let read = &*fs::read_to_string(path)?;
            return Ok(serde_json::from_str(read)?)
        }
        return Ok(Validator::new())
    }

    #[inline]
    fn new() -> Validator {
        Validator {
            certificates: HashMap::new()
        }
    }

    pub(crate) fn add(&mut self, name: String, certificate: Certificate) {
        self.certificates.insert(name, certificate);
    }

    pub(crate) async fn compare(&self, name: &String) -> bool {
        if self.certificates.contains_key(name) {
            let certificate = self.certificates.get(name).unwrap();

            let path = &certificate.path;
            if Path::new(path).exists() {
                let hash = hash_file(&certificate.path).await.unwrap();
                return certificate.hash == hash;
            }
        }
        return false;
    }
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub(crate) struct Certificate {
    hash: String,
    path: String
}

impl Certificate {
    #[inline]
    pub(crate) fn new(hash: String, path: &PathBuf) -> Certificate {
        Certificate {
            hash,
            path: path.to_str().unwrap().to_string(),
        }
    }
}

pub(crate) async fn hash_file<P: AsRef<Path>>(path: P) -> Option<String> {
    let read = fs::read(path).expect("Failed to read file");
    let mut hasher = Sha256::new();

    hasher.update(&read.as_slice());
    let hash = hasher.finalize();
    Some(format!("{:x}", hash))
}