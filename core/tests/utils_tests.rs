use pakdo_core::errors::PakdoError;
use pakdo_core::utils::get_extension_from_file;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_get_extension_known_png() {
    let mut f = NamedTempFile::new().unwrap();
    // PNG magic bytes signature
    f.write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
        .unwrap();
    let result = get_extension_from_file(f.path());
    let ext = result.expect("should detect PNG extension");
    assert_eq!(ext, "png");
}

#[test]
fn test_get_extension_known_jpg() {
    let mut f = NamedTempFile::new().unwrap();
    // JPEG magic bytes signature
    f.write_all(&[0xFF, 0xD8, 0xFF]).unwrap();
    let result = get_extension_from_file(f.path());
    let ext = result.expect("should detect JPG extension");
    assert_eq!(ext, "jpg");
}

#[test]
fn test_get_extension_file_not_found() {
    let dir = TempDir::new().unwrap();
    let missing = dir.path().join("missing.xyz");
    assert!(!missing.exists());
    let result = get_extension_from_file(&missing);
    assert!(matches!(result, Err(PakdoError::FileNotFound(_))));
}

#[test]
fn test_get_extension_unknown_type() {
    // Empty file — infer cannot determine the type
    let f = NamedTempFile::new().unwrap();
    let result = get_extension_from_file(f.path());
    assert!(matches!(result, Err(PakdoError::UnknownFileExtension(_))));
}
