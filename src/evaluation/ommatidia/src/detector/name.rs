use std::{fmt::Display, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidName;

impl Display for InvalidName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("name must be valid ASCII and may contain lowercase and uppercase letters, digits, underscores, periods and dashes. It must not start with a period or a dash and may contain a maximum of 128 characters")
    }
}

impl std::error::Error for InvalidName {}

/// The name of a detector checked accordingly to Dockers regulations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct Name(Rc<String>);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for Name {
    type Error = InvalidName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        // ToDo: Check for period/dash at the beginning.
        match name.chars().all(|char| {
            char.is_ascii_alphanumeric() || char == '-' || char == '_' || char == '.' || char == '/'
        }) && (1..=128).contains(&name.len())
        {
            true => Ok(Name(Rc::new(name))),
            false => Err(InvalidName),
        }
    }
}

impl<'a> TryFrom<&'a str> for Name {
    type Error = InvalidName;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl serde::Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{InvalidName, Name};

    #[test]
    fn test_valid() {
        let name = Name::try_from("detector-123").expect("valid name");
        assert_eq!(name.as_ref(), "detector-123");
        assert_eq!(name.to_string(), "detector-123");
    }

    #[test]
    fn test_size() {
        assert_eq!(Name::try_from(""), Err(InvalidName));
        assert_eq!(
            Name::try_from((0..129).map(|_| 'A').collect::<String>()),
            Err(InvalidName)
        );
    }

    #[test]
    fn test_invalid_char() {
        assert_eq!(Name::try_from("ä"), Err(InvalidName));
    }
}
