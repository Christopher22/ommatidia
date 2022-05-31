#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ErrorPrinter;

impl ommatidia::ErrorHandler for ErrorPrinter {
    fn handle<E: std::error::Error>(&self, error: E) {
        eprintln!("{}", error)
    }
}
