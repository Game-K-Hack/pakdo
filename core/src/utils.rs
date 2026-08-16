use crate::errors::PakdoError;
use std::path::Path;

pub fn get_extension_from_file(path: &Path) -> Result<&str, PakdoError> {
    match infer::get_from_path(path) {
        Ok(Some(ext)) => Ok(ext.extension()),
        Ok(None) => Err(PakdoError::UnknownFileExtension(
            path.to_string_lossy().into_owned(),
        )),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(PakdoError::FileNotFound(path.to_string_lossy().into_owned()))
        }
        Err(e) => Err(PakdoError::FailedToOpenFile(
            path.to_string_lossy().into_owned(),
            e,
        )),
    }
}
