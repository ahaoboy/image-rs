use std::io::{Cursor, Read};

use image::{guess_format, GenericImageView, ImageDecoder};
use wasm_bindgen::prelude::wasm_bindgen;

use image::io::Reader as ImageReader;

#[wasm_bindgen]
pub struct Image {
    _pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub channel: u32,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(getter)]
    pub fn pixels(&self) -> Vec<u8> {
        self._pixels.clone()
    }
}

#[wasm_bindgen]
pub fn decode(buf: Vec<u8>) -> Image {
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
