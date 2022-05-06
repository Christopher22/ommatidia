use std::{path::Path, rc::Rc};

use bytes::Bytes;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    content: Bytes,
    identifier: Rc<String>,
}

impl Sample {
    pub async fn from_path<S: AsRef<str>, P: AsRef<Path>>(
        name: S,
        path: P,
    ) -> std::io::Result<Sample> {
        let path = path.as_ref();
        let mut file = File::open(path).await?;
        let length = file.metadata().await?.len();

        let mut data = Vec::with_capacity(length.try_into().expect("smaller file size"));
        file.read_to_end(&mut data).await?;

        Ok(Self {
            content: Bytes::from(data),
            identifier: Rc::new(Self::create_identifier(name, path)),
        })
    }

    pub fn create_identifier<S: AsRef<str>, P: AsRef<Path>>(name: S, path: P) -> String {
        format!("{}:{}", name.as_ref(), path.as_ref().display())
    }
}

impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.identifier.as_str())
    }
}

impl AsRef<str> for Sample {
    fn as_ref(&self) -> &str {
        &self.identifier
    }
}

impl AsRef<[u8]> for Sample {
    fn as_ref(&self) -> &[u8] {
        &self.content
    }
}

impl From<Sample> for Bytes {
    fn from(sample: Sample) -> Self {
        sample.content
    }
}

#[cfg(test)]
mod tests {
    use super::Sample;
    use std::{io::Write, path::PathBuf};

    #[test]
    fn test_identifier() {
        let identifier = Sample::create_identifier("test", PathBuf::from("/dir/file"));
        assert_eq!(identifier, "test:/dir/file");
    }

    #[tokio::test]
    async fn test_loading() {
        let content = [42u8, 43, 44];

        let mut file = tempfile::NamedTempFile::new().expect("valid temp file");
        file.as_file_mut()
            .write_all(&content)
            .expect("writing data");

        let sample = Sample::from_path("test", file.path())
            .await
            .expect("opening failed");

        assert_eq!(content, sample.as_ref());
    }
}
