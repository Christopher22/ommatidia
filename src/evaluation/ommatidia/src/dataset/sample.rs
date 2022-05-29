use std::path::Path;

use bytes::Bytes;
use tokio::{fs::File, io::AsyncReadExt};

use super::Identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    pub content: Bytes,
    pub identifier: Identifier,
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
            identifier: Identifier::from_path(name, path),
        })
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
    use std::io::Write;

    use super::Sample;

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
