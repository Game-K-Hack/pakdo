use crate::modules::converter::Converter;
use crate::modules::image::Image;
use phf::phf_map;
use std::path::Path;
pub static ROUTES: phf::Map<
    &'static str,
    fn(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>>,
> = phf_map! {
    "jpg>png" => Image::process,
    "png>jpg" => Image::process,
};
