use crate::config::ROUTES;
use crate::errors::PakdoError;
use crate::utils::get_extension_from_file;
use std::path::{Path, PathBuf};

mod config;
pub mod errors;
pub mod modules;
pub mod utils;

pub fn convert(
    input_path: &Path,
    output_path: Option<&Path>,
    target_extension: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let destination_path: PathBuf = match (output_path, target_extension) {
        (Some(output_path), None) => output_path.to_path_buf(),
        (None, Some(target_extension)) => input_path.with_extension(target_extension),
        (Some(_), Some(_)) => {
            return Err(Box::from(PakdoError::InvalidArguments(
                "You must provide either an output path or a target extension, but not both."
                    .to_string(),
            )));
        }
        (None, None) => {
            return Err(Box::from(PakdoError::InvalidArguments(
                "You must provide either an output path or a target extension.".to_string(),
            )));
        }
    };

    convert_file(input_path, &destination_path)?;

    Ok(())
}

fn convert_file(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let conversion_string = get_conversion_string(input_path, output_path)?;

    if let Some(conversion_function) = ROUTES.get(conversion_string.as_str()) {
        conversion_function(input_path, output_path)?;
        Ok(())
    } else {
        Err(Box::from(PakdoError::ExtensionNotSupported(
            conversion_string,
        )))
    }
}

fn get_conversion_string(input_path: &Path, output_path: &Path) -> Result<String, PakdoError> {
    let input_file_extension = get_extension_from_file(input_path)?.to_lowercase();
    let output_ext = output_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| PakdoError::UnknownFileExtension(output_path.to_string_lossy().to_string()))?
        .to_lowercase();

    Ok(format!("{input_file_extension}>{output_ext}"))
}
