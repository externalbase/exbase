use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io {
        source: std::io::Error,
        path: Option<String>
    },
    Os {
        source: std::io::Error,
        syscall: String
    },
    Other {
        text: String
    }
}

#[cfg(feature = "ffi")]
pub enum ErrorFFI {
    NullPointer { obj: String },
    Utf8Error(std::str::Utf8Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io { source: e, path: None }
    }
}

impl Error {
    pub fn io(source: std::io::Error, path: String) -> Self {
        Error::Io {
            source,
            path: Some(path),
        }
    }

    pub fn os(syscall: &str) -> Self {
        Error::Os {
            source: std::io::Error::last_os_error(),
            syscall: syscall.to_owned()
        }
    }

    pub fn other(text: &str) -> Self {
        Error::Other {
            text: text.to_owned()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { source, path } => {
                match path {
                    Some(path) => write!(f, "I/O error on '{}': '{}'", path, source),
                    None => write!(f, "I/O error: {}", source),
                }
            },
            Error::Os { source, syscall } => write!(f, "OS error '{}' in '{}'", source, syscall),
            Error::Other { text } => write!(f, "{text}"),
        }
    }
}

#[cfg(feature = "ffi")]
impl fmt::Display for ErrorFFI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorFFI::NullPointer { obj } => write!(f, "Null pointer ({obj})"),
            ErrorFFI::Utf8Error(e) => write!(f, "{e}"),
        }
    }
}