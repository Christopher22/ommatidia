use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "PathBuf")]
pub struct ReadableFilePath(PathBuf);

impl TryFrom<PathBuf> for ReadableFilePath {
    type Error = &'static str;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        match File::open(&path).is_ok() {
            true => Ok(ReadableFilePath(path)),
            false => Err("Unable to open specified file"),
        }
    }
}

impl AsRef<Path> for ReadableFilePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
