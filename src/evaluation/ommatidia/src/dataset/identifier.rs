use std::{path::Path, rc::Rc};

/// The unique identifier of a sample which is cheaply clonable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(Rc<String>);

impl Identifier {
    pub fn from_path<S: AsRef<str>, P: AsRef<Path>>(dataset: S, path: P) -> Self {
        Identifier(format!("{}:{}", dataset.as_ref(), path.as_ref().display()).into())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<&str> for Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(identifier: &'a str) -> Self {
        Self(identifier.to_owned().into())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Identifier;

    #[test]
    fn test_identifier() {
        let identifier = Identifier::from_path("test", PathBuf::from("/dir/file"));
        assert_eq!(identifier, "test:/dir/file");
    }
}
