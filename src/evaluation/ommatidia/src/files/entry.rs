use std::path::PathBuf;

use glob::Paths;

pub enum Entry {
    Path(PathBuf),
    Pattern(Paths),
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path(path) => f.debug_tuple("Path").field(path).finish(),
            Self::Pattern(_) => f.debug_tuple("Pattern").finish(),
        }
    }
}

impl From<PathBuf> for Entry {
    fn from(path: PathBuf) -> Self {
        Entry::Path(path)
    }
}

impl<'de> serde::Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EntryVisitor)
    }
}

struct EntryVisitor;

impl EntryVisitor {
    fn try_load<E: serde::de::Error, S: AsRef<str>>(value: S) -> Result<Entry, E> {
        let value = value.as_ref();
        match value.contains(['*', '?', '[', '[']) {
            true => match glob::glob(value) {
                Ok(pattern) => Ok(Entry::Pattern(pattern)),
                Err(error) => Err(E::custom(error)),
            },
            false => Ok(PathBuf::from(value).into()),
        }
    }
}

impl<'de> serde::de::Visitor<'de> for EntryVisitor {
    type Value = Entry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an path with or without wildcards")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::try_load(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::try_load(v)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::try_load(v)
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_deserialize_path() {
        let v: Entry = serde_json::from_str("\"/dir/test\"").expect("valid json");
        assert!(matches!(v, Entry::Path(_)));
    }

    #[test]
    fn test_deserialize_pattern() {
        let v: Entry = serde_json::from_str("\"/dir/*/test\"").expect("valid json");
        assert!(matches!(v, Entry::Pattern(_)));
    }
}
