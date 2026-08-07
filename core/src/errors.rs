use thiserror::Error;

#[derive(Error, Debug)]
pub enum PakdoError {
    #[error("Unknown file extension for the file: {0}")]
    UnknownFileExtension(String),
    #[error("Unknown library: {0}")]
    UnknownLibrary(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Failed to open file: {0}")]
    FailedToOpenFile(String, #[source] std::io::Error),
    #[error("Extension not supported: {0}")]
    ExtensionNotSupported(String),
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
}
