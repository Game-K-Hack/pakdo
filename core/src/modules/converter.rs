use std::error::Error;
use std::path::Path;

pub trait Converter {
    fn process(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>>;
}
