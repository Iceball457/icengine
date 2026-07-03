#[derive(Debug)]
pub struct Error {
    fatal: bool,
    error: anyhow::Error,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { fatal, error } = self;
        if *fatal {
            write!(f, "FATAL: {error}")
        } else {
            write!(f, "{error}")
        }
    }
}

impl Error {
    #[must_use]
    pub const fn new(error: anyhow::Error) -> Self {
        Self {
            fatal: false,
            error,
        }
    }

    #[must_use]
    pub const fn new_fatal(error: anyhow::Error) -> Self {
        Self {
            fatal: false,
            error,
        }
    }

    #[must_use]
    pub const fn fatal(&self) -> bool {
        self.fatal
    }
}
