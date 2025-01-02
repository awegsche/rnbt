

#[derive(Debug)]
pub enum NbtError {
    RootNotCompoundError,
    IOError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for NbtError {
    fn from(value: std::io::Error) -> Self {
        NbtError::IOError(value)
    }
}

impl From<std::string::FromUtf8Error> for NbtError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        NbtError::Utf8Error(value)
    }
}
