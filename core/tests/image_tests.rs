use pakdo_core::modules::converter::Converter;
use pakdo_core::modules::image::Image;
use tempfile::TempDir;

// Happy-path format conversions (jpg→png, png→jpg) are already covered
// end-to-end in convert_tests.rs. Tests here should only cover behavior
// specific to the Image converter that cannot be observed through convert().
//
// Add tests here when Image supports formats not registered in ROUTES
// (i.e., formats only reachable via the --lib flag).

#[test]
fn test_process_nonexistent_file_returns_error() {
    // Verifies that Image::process returns an error when the input file does not exist.
    // Tested at the converter level (not via convert()) to ensure the error
    // originates from the converter itself, not from the routing layer.
    let dir = TempDir::new().unwrap();
    let missing_input = dir.path().join("does_not_exist.jpg");
    assert!(!missing_input.exists());
    let output = dir.path().join("out.png");
    let result = Image::process(&missing_input, &output);
    let err = result.expect_err("expected an error for nonexistent file");
    // Image::process uses ImageReader::open() which returns an io::Error
    // when the file does not exist.
    let io_err = err
        .downcast_ref::<std::io::Error>()
        .expect("error should be an io::Error");
    assert_eq!(io_err.kind(), std::io::ErrorKind::NotFound);
}
