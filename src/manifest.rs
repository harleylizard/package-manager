use std::path::PathBuf;
use std::process::Command;

use reqwest::Url;
use serde::Deserialize;

use crate::device;

#[derive(Deserialize)]
pub(crate) struct Dependency {
    pub(crate) name: String,
    url: String
}

impl Dependency {
    #[inline]
    pub(crate) fn get_url(&self) -> Result<Url, url::ParseError> {
        Url::parse(&*self.url)
    }

    #[inline]
    pub(crate) fn get_path(&self, platform: &Platform) -> PathBuf {
        let path = format!("{}/{}.jar", platform.directory, self.name);
        PathBuf::from(path)
    }
}

#[derive(Deserialize)]
struct Java {
    jre: String,
    url: String,
    main: String,
    command: String,
    args: Vec<String>
}

#[derive(Deserialize)]
pub(crate) struct Platform {
    os: device::OS,
    arch: device::Arch,
    java: Java,
    directory: String,
    pub(crate) dependencies: Vec<Dependency>
}

impl Platform {
    #[inline]
    pub(crate) fn get_java_url(&self) -> Result<Url, url::ParseError> {
        Url::parse(&*self.java.url)
    }

    #[inline]
    pub(crate) fn get_java_path(&self) -> PathBuf {
        PathBuf::from(&self.java.jre)
    }

    #[inline]
    pub(crate) fn get_java_folder(&self) -> &String {
       return &self.java.jre
    }

    pub(crate) fn get_command(&self) -> Command {
        let split: Vec<&str> = self.java.command.split_whitespace().collect();
        let program = split[0];

        let mut command = Command::new(program.trim());
        let mut args = Vec::new();
        for str in split.iter().skip(1) {
            let formatted = str
                .replace("$jre", &self.java.jre)
                .replace("$directory", &self.directory)
                .replace("$main", &self.java.main);
            args.push(formatted);
        }
        for str in &args {
            println!("{}", str);
        }
        command.args(args);

        return command;
    }
}

#[derive(Deserialize)]
pub(crate) struct Manifest {
    #[serde(rename = "platform")]
    platforms: Vec<Platform>
}

impl Manifest {
    pub(crate) fn get_platform(&self) -> Option<&Platform> {
        let device = device::Device::get_specific();
        for platform in &self.platforms {
            if platform.os == device.os && platform.arch == device.arch {
                return Some(platform)
            }
        }
        return None
    }
}