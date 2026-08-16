mod common;

use common::{downcast_pakdo_error, fixture_path};
use pakdo_core::convert;
use pakdo_core::errors::PakdoError;
use std::path::Path;
use tempfile::TempDir;

// --- Error cases ---

#[test]
fn test_convert_no_output_no_format_returns_error() {
    let err = downcast_pakdo_error(convert(Path::new("any.jpg"), None, None, None));
    assert!(matches!(err, PakdoError::InvalidArguments(_)));
}

#[test]
fn test_convert_unknown_library_returns_error() {
    let dir = TempDir::new().unwrap();
    let output = dir.path().join("out.jpg");
    let err = downcast_pakdo_error(convert(
        &fixture_path("image.png"),
        Some(&output),
        None,
        Some("nonexistent_lib"),
    ));
    assert!(matches!(
        err,
        PakdoError::UnknownLibrary(lib) if lib == "nonexistent_lib"
    ));
}

#[test]
fn test_convert_unsupported_route_returns_error() {
    // png->gif is not registered in ROUTES
    let dir = TempDir::new().unwrap();
    let output = dir.path().join("out.gif");
    let err = downcast_pakdo_error(convert(
        &fixture_path("image.png"),
        Some(&output),
        None,
        None,
    ));
    assert!(matches!(err, PakdoError::ExtensionNotSupported(_)));
}

#[test]
fn test_convert_output_file_with_extension_and_target_ext_returns_error() {
    let dir = TempDir::new().unwrap();
    let output_file = dir.path().join("out.png"); // has an extension
    let err = downcast_pakdo_error(convert(
        &fixture_path("image.jpg"),
        Some(&output_file),
        Some("jpg"), // also providing target_extension
        None,
    ));
    assert!(matches!(err, PakdoError::InvalidArguments(_)));
}

// --- Happy path ---

#[test]
fn test_convert_jpg_to_png_with_output_path() {
    let dir = TempDir::new().unwrap();
    let output = dir.path().join("out.png");
    let result = convert(&fixture_path("image.jpg"), Some(&output), None, None);
    result.expect("conversion jpg->png with output path should succeed");
    assert!(output.exists());
}

#[test]
fn test_convert_png_to_jpg_with_output_dir_and_extension() {
    // Passing an output directory + target extension: convert should build
    // the output filename from the input stem (image.png -> image.jpg).
    let dir = TempDir::new().unwrap();
    let output_dir = dir.path();
    let result = convert(
        &fixture_path("image.png"),
        Some(output_dir),
        Some("jpg"),
        None,
    );
    result.expect("conversion png->jpg with output directory and extension should succeed");
    assert!(output_dir.join("image.jpg").exists());
}
