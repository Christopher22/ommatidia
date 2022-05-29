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
