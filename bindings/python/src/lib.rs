use pakdo_core::errors::PakdoError;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::path::Path;

// Exception hierarchy: PakdoError (base) with specific subclasses
create_exception!(
    pakdo,
    PakdoBaseError,
    PyException,
    "Base exception for all pakdo errors."
);
create_exception!(
    pakdo,
    InvalidArgumentsError,
    PakdoBaseError,
    "Invalid arguments provided."
);
create_exception!(
    pakdo,
    FileNotFoundError,
    PakdoBaseError,
    "Input file not found."
);
create_exception!(pakdo, FileOpenError, PakdoBaseError, "Failed to open file.");
create_exception!(
    pakdo,
    UnknownFileExtensionError,
    PakdoBaseError,
    "Could not detect file extension."
);
create_exception!(
    pakdo,
    UnknownLibraryError,
    PakdoBaseError,
    "Unknown conversion library."
);
create_exception!(
    pakdo,
    ExtensionNotSupportedError,
    PakdoBaseError,
    "Conversion route not supported."
);

/// Convert a `PakdoError` into the matching Python exception.
fn to_py_err(err: PakdoError) -> PyErr {
    let msg = err.to_string();
    match err {
        PakdoError::InvalidArguments(_) => InvalidArgumentsError::new_err(msg),
        PakdoError::FileNotFound(_) => FileNotFoundError::new_err(msg),
        PakdoError::FailedToOpenFile(_, _) => FileOpenError::new_err(msg),
        PakdoError::UnknownFileExtension(_) => UnknownFileExtensionError::new_err(msg),
        PakdoError::UnknownLibrary(_) => UnknownLibraryError::new_err(msg),
        PakdoError::ExtensionNotSupported(_) => ExtensionNotSupportedError::new_err(msg),
    }
}

/// Convert a file from one format to another.
///
/// Args:
///     input: Path to the input file.
///     output: Path to the output file or directory (optional).
///     format: Target format extension, e.g. "png" (optional).
///     lib: Library to use for conversion, e.g. "image" (optional).
///
/// Raises:
///     PakdoBaseError: Base class for all pakdo errors.
///     InvalidArgumentsError: If required arguments are missing or conflicting.
///     FileNotFoundError: If the input file does not exist.
///     UnknownLibraryError: If the specified library is not recognized.
///     ExtensionNotSupportedError: If the conversion route is not supported.
#[pyfunction]
#[pyo3(signature = (input, *, output=None, format=None, lib=None))]
fn convert(
    input: &str,
    output: Option<&str>,
    format: Option<&str>,
    lib: Option<&str>,
) -> PyResult<()> {
    let input_path = Path::new(input);
    let output_path = output.map(Path::new);

    pakdo_core::convert(input_path, output_path, format, lib).map_err(|e| {
        match e.downcast::<PakdoError>() {
            Ok(pakdo_err) => to_py_err(*pakdo_err),
            Err(other) => PakdoBaseError::new_err(other.to_string()),
        }
    })
}

/// Pakdo: optimized, 100% local file converter.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;

    // Register exception types so users can catch them
    m.add("PakdoBaseError", m.py().get_type::<PakdoBaseError>())?;
    m.add(
        "InvalidArgumentsError",
        m.py().get_type::<InvalidArgumentsError>(),
    )?;
    m.add("FileNotFoundError", m.py().get_type::<FileNotFoundError>())?;
    m.add("FileOpenError", m.py().get_type::<FileOpenError>())?;
    m.add(
        "UnknownFileExtensionError",
        m.py().get_type::<UnknownFileExtensionError>(),
    )?;
    m.add(
        "UnknownLibraryError",
        m.py().get_type::<UnknownLibraryError>(),
    )?;
    m.add(
        "ExtensionNotSupportedError",
        m.py().get_type::<ExtensionNotSupportedError>(),
    )?;

    Ok(())
}
