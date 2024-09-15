use std::io::Cursor;
use common::Image;
use wasm_bindgen::prelude::wasm_bindgen;
use image::ImageReader;
pub mod encode;
pub mod common;


#[wasm_bindgen]
pub fn decode(buf: &[u8]) -> Image {
    let img = ImageReader::new(Cursor::new(buf))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let width = img.width();
    let height = img.height();
    let channel = match img.color() {
        image::ColorType::Rgb8 => 3,
        image::ColorType::Rgba8 => 4,
        _ => 4,
    };

    let p = match channel {
        3 => img.to_rgb8().to_vec(),
        4 => img.to_rgba8().to_vec(),
        _ => img.to_rgba8().to_vec(),
    };
    Image {
        width,
        height,
        _pixels: p,
        channel,
    }
}
