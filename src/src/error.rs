
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io {
        source: std::io::Error,
        path: String
    },
    Os {
        source: std::io::Error,
        syscall: String
    }
}

impl Error {
    pub fn io(source: std::io::Error, path: String) -> Self {
        Error::Io {
            source,
            path,
        }
    }

    pub fn os(syscall: &str) -> Self {
        Error::Os {
            source: std::io::Error::last_os_error(),
            syscall: syscall.to_owned()
        }
    }
}

// todo impl fmt
