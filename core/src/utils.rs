use crate::errors::PakdoError;
use std::path::Path;

pub fn get_extension_from_file(path: &Path) -> Result<&str, PakdoError> {
    let path_str = path.to_string_lossy().to_string();

    match infer::get_from_path(path) {
        Ok(Some(ext)) => Ok(ext.extension()),
        Ok(None) => Err(PakdoError::UnknownFileExtension(path_str)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(PakdoError::FileNotFound(path_str)),
        Err(e) => Err(PakdoError::FailedToOpenFile(path_str, e)),
    }
}
