use std::fmt::Display;

pub trait CommandReport {
    fn name(&self) -> &'static str;

    fn report_error(&self, path: Option<&str>, err: impl Display) {
        match path {
            Some(p) => eprintln!("{}: {}: {}", self.name(), p, err),
            None => eprintln!("{}: {}", self.name(), err),
        }
    }
}
