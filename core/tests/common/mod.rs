use pakdo_core::errors::PakdoError;
use std::path::{Path, PathBuf};

pub fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Unwrap an error and downcast it to PakdoError, panicking with a clear
/// message if the downcast fails.
pub fn downcast_pakdo_error(result: Result<(), Box<dyn std::error::Error>>) -> PakdoError {
    let err = result.expect_err("expected an error");
    *err.downcast::<PakdoError>()
        .expect("error should downcast to PakdoError")
}
