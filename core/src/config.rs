use crate::modules::converter::Converter;
use crate::modules::image::Image;
use phf::phf_map;
use std::path::Path;

type ConvertFn = fn(&Path, &Path) -> Result<(), Box<dyn std::error::Error>>;

pub static LIBRARY: phf::Map<&'static str, ConvertFn> = phf_map! {
    "image" => Image::process,
};

pub static ROUTES: phf::Map<&'static str, ConvertFn> = phf_map! {
    "jpg>png" => Image::process,
    "png>jpg" => Image::process,
    "webp>jpg" => Image::process,
    "webp>png" => Image::process,
    "png>webp" => Image::process,
    "jpg>webp" => Image::process,
};
