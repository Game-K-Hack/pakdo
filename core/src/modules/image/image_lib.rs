use crate::modules::converter::Converter;
use image::ImageReader;
use std::path::Path;

pub struct Image;
impl Converter for Image {
    fn process(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let input_image = ImageReader::open(input_path)?.decode()?;
        input_image.save(output_path)?;
        Ok(())
    }
}
