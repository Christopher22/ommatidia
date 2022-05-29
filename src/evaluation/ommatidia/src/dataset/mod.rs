mod entry;
mod error;
mod identifier;
mod sample;

use std::path::PathBuf;

use crate::ErrorHandler;

pub use self::{
    entry::Entry,
    error::{Error, ErrorType},
    identifier::Identifier,
    sample::Sample,
};

pub type Samples = async_broadcast::Receiver<Sample>;

#[derive(serde::Deserialize)]
pub struct Dataset {
    pub name: String,
    pub patterns: Vec<Entry>,
    #[serde(skip)]
    buffer: Buffer,
}

impl Dataset {
    pub fn new<S: Into<String>>(name: S, patterns: Vec<Entry>) -> Self {
        Self {
            name: name.into(),
            patterns,
            buffer: Buffer::default(),
        }
    }

    pub async fn load<E: ErrorHandler>(mut self, error_handler: &E) {
        // Swap patterns out of object to be able to iterate through entities while calling "load_sample"
        let mut patterns = Vec::new();
        std::mem::swap(&mut self.patterns, &mut patterns);

        for entry in patterns {
            match entry {
                Entry::Path(path) => {
                    if !self.load_sample(path, error_handler).await {
                        return;
                    }
                }
                Entry::Pattern(paths) => {
                    // Iterate through all the paths
                    'inner: for path in paths {
                        match path {
                            Ok(path) => {
                                if !self.load_sample(path, error_handler).await {
                                    return;
                                }
                            }
                            Err(error) => {
                                error_handler.handle(Error {
                                    dataset: self.name.clone(),
                                    details: error.into(),
                                });
                                continue 'inner;
                            }
                        };
                    }
                }
            };
        }
    }

    pub fn create_connection(&mut self) -> Samples {
        match self.buffer.output.take() {
            Some(value) => value,
            None => self.buffer.input.new_receiver(),
        }
    }

    #[allow(clippy::needless_return)]
    async fn load_sample<E: ErrorHandler>(&self, path: PathBuf, error_handler: &E) -> bool {
        match Sample::from_path(&self.name, &path).await {
            Ok(sample) => {
                if self.buffer.input.broadcast(sample).await.is_err() {
                    error_handler.handle(Error {
                        dataset: self.name.clone(),
                        details: ErrorType::Aborted,
                    });
                    return false;
                }
            }
            Err(error) => error_handler.handle(Error {
                dataset: self.name.clone(),
                details: ErrorType::Io(Identifier::from_path(&self.name, path), error),
            }),
        };
        return true;
    }
}

impl std::fmt::Debug for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Files")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
struct Buffer {
    input: async_broadcast::Sender<Sample>,
    output: Option<Samples>,
}

impl Default for Buffer {
    fn default() -> Self {
        let (input, output) = async_broadcast::broadcast(16);
        Self {
            input,
            output: Some(output),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::{Dataset, Entry};

    #[derive(Debug, Clone, Copy, Default)]
    struct NoErrorAssumption;
    impl crate::ErrorHandler for NoErrorAssumption {
        fn handle<E: std::error::Error>(&self, error: E) {
            panic!("Unexpected error occured: {}", error)
        }
    }

    #[tokio::test]
    async fn test_loading() {
        let content = [42u8, 43, 44];
        let mut temp_file = tempfile::NamedTempFile::new().expect("valid temp file");
        temp_file
            .as_file_mut()
            .write_all(&content)
            .expect("writing files");

        let mut files = Dataset::new("test", vec![Entry::Path(temp_file.path().to_owned())]);
        let mut output = files.create_connection();

        // Load files filling the buffer
        files.load(&NoErrorAssumption::default()).await;

        // Check values loaded
        assert_eq!(output.recv().await.expect("valid value").as_ref(), content);
        assert_eq!(output.recv().await, Err(async_broadcast::RecvError::Closed));
    }
}
