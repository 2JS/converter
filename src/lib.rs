extern crate image;

// use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn jpeg_png(infile_path: &str, outfile_path: &str) {
    let image = image::open(infile_path).unwrap();

    image.save(outfile_path).unwrap();
}