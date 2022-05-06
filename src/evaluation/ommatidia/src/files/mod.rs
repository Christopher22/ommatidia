mod entry;
mod error;
mod sample;

pub use self::{entry::Entry, error::Error, sample::Sample};

use super::Errors;

pub type Samples = async_broadcast::Receiver<Sample>;

#[derive(serde::Deserialize)]
pub struct Files {
    pub name: String,
    pub patterns: Vec<Entry>,
    #[serde(skip)]
    buffer: Buffer,
}

impl Files {
    pub fn new<S: Into<String>>(name: S, patterns: Vec<Entry>) -> Self {
        Self {
            name: name.into(),
            patterns,
            buffer: Buffer::default(),
        }
    }

    pub async fn load(self) -> Errors<Error> {
        let mut errors = Errors::default();

        for entry in self.patterns {
            match entry {
                Entry::Path(path) => {
                    if let Ok(sample) = Sample::from_path(&self.name, &path).await.map_err(|e| {
                        errors
                            .with_id(Sample::create_identifier(&self.name, path))
                            .report(e)
                    }) {
                        if self.buffer.input.broadcast(sample).await.is_err() {
                            return errors;
                        }
                    }
                }
                Entry::Pattern(paths) => {
                    // Iterate through all the paths
                    for path in paths {
                        let path = match path.map_err(|e| errors.report(e)) {
                            Ok(path) => path,
                            Err(_) => continue,
                        };

                        if let Ok(sample) =
                            Sample::from_path(&self.name, &path).await.map_err(|e| {
                                errors
                                    .with_id(Sample::create_identifier(&self.name, &path))
                                    .report(e)
                            })
                        {
                            if self.buffer.input.broadcast(sample).await.is_err() {
                                return errors;
                            }
                        }
                    }
                }
            };
        }

        errors
    }

    pub fn create_connection(&mut self) -> Samples {
        match self.buffer.output.take() {
            Some(value) => value,
            None => self.buffer.input.new_receiver(),
        }
    }
}

impl std::fmt::Debug for Files {
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

    use super::{Entry, Files};

    #[tokio::test]
    async fn test_loading() {
        let content = [42u8, 43, 44];
        let mut temp_file = tempfile::NamedTempFile::new().expect("valid temp file");
        temp_file
            .as_file_mut()
            .write_all(&content)
            .expect("writing files");

        let mut files = Files::new("test", vec![Entry::Path(temp_file.path().to_owned())]);
        let mut output = files.create_connection();

        // Load files filling the buffer
        let errors = files.load().await;
        assert!(!errors.any_errors());

        // Check values loaded
        assert_eq!(output.recv().await.expect("valid value").as_ref(), content);
        assert_eq!(output.recv().await, Err(async_broadcast::RecvError::Closed));
    }
}
