use crate::config::{LIBRARY, ROUTES};
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
    library: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let destination_path: PathBuf = match (output_path, target_extension) {
        (Some(output_path), None) => output_path.to_path_buf(),
        (None, Some(target_extension)) => input_path.with_extension(target_extension),
        (Some(output), Some(ext)) => {
            if output.is_dir() || (!output.exists() && output.extension().is_none()) {
                let output_file_name = input_path.with_extension(ext);
                let file_name = output_file_name.file_name().ok_or_else(|| {
                    PakdoError::InvalidArguments(
                        "Cannot determine the file name from the input path.".into(),
                    )
                })?;
                output.join(file_name)
            } else {
                return Err(Box::from(PakdoError::InvalidArguments(
                    "When providing a target extension, the output path must be a directory."
                        .into(),
                )));
            }
        }
        (None, None) => {
            return Err(Box::from(PakdoError::InvalidArguments(
                "You must provide either an output path or a target extension.".into(),
            )));
        }
    };

    convert_file(input_path, &destination_path, library)?;

    Ok(())
}

fn convert_file(
    input_path: &Path,
    output_path: &Path,
    library: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let conversion_string = get_conversion_string(input_path, output_path)?;

    let conversion_function = match library {
        Some(lib_name) => *LIBRARY
            .get(lib_name)
            .ok_or_else(|| PakdoError::UnknownLibrary(lib_name.to_string()))?,

        None => *ROUTES
            .get(conversion_string.as_str())
            .ok_or_else(|| PakdoError::ExtensionNotSupported(conversion_string))?,
    };

    conversion_function(input_path, output_path)
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
