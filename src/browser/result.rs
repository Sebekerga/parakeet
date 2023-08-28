/// Shorthand for `Result<T, browser::result::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the browser module
pub struct Error {
    pub stage: &'static str,
    pub message: String,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "at {}: {}", self.stage, self.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.stage, self.message)
    }
}

macro_rules! error {
    ($stage:expr, $($arg:tt)*) => {
        crate::browser::result::Error {
            stage: $stage,
            message: format!($($arg)*),
        }
    };
}

pub(crate) use error;
