pub fn check_duplicates<T, C: Fn(&T) -> &str>(
    mut config: Vec<T>,
    callback: C,
) -> Result<Vec<T>, String> {
    let mut duplicate = None;
    config.sort_unstable_by(|a, b| callback(a).cmp(callback(b)));
    config.dedup_by(|a, b| {
        let equality = callback(a) == callback(b);
        if equality {
            duplicate = Some(callback(a).to_owned());
        }
        equality
    });

    match duplicate {
        None => Ok(config),
        Some(duplicate_name) => Err(duplicate_name),
    }
}
