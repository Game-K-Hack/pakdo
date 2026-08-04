use crate::modules::converter::Converter;
use std::path::Path;

pub struct Image;
impl Converter for Image {
    fn process(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
