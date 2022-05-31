use std::any::Any;

struct RawError<T: Any + AsRef<str>>(pub T);

impl<T: Any + AsRef<str>> std::fmt::Debug for RawError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RawError").finish()
    }
}

impl<T: Any + AsRef<str>> std::fmt::Display for RawError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

impl<T: Any + AsRef<str>> std::error::Error for RawError<T> {}

pub trait ErrorHandler {
    fn handle<E: std::error::Error + Any>(&self, error: E);

    /// Report a raw error in the shape of plain text
    fn handle_raw<E: 'static + AsRef<str>>(&self, error: E) {
        self.handle(RawError(error))
    }
}

pub fn check_duplicates<T, O: ?Sized + Ord + ToOwned, C: Fn(&T) -> &O>(
    mut config: Vec<T>,
    callback: C,
) -> Result<Vec<T>, O::Owned> {
    let mut duplicate = None;
    config.sort_unstable_by(|a, b| callback(a).cmp(callback(b)));
    config.dedup_by(|a, b| {
        let mapped_a = callback(a);
        let equality = mapped_a == callback(b);
        if equality {
            duplicate = Some(mapped_a.to_owned());
        }
        equality
    });

    match duplicate {
        None => Ok(config),
        Some(duplicate_name) => Err(duplicate_name),
    }
}
