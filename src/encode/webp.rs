use std::io::Cursor;
use image::{ExtendedColorType, ImageDecoder};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::common::Image;

#[wasm_bindgen]
pub fn encode_webp(pixels: &[u8], width: u32, height: u32, channel: u32) -> Vec<u8> {
    let mut v = vec![];
    let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut v);
    let color_type = match channel {
        3 => ExtendedColorType::Rgb8,
        4 => ExtendedColorType::Rgba8,
        _ => {
            panic!("channel must be 3 or 4")
        }
    };
    encoder.encode(pixels, width, height, color_type).unwrap();
    v
}

#[wasm_bindgen]
pub fn decode_webp(pixels: &[u8]) -> Image {
    let decoder = image::codecs::webp::WebPDecoder::new(Cursor::new(pixels)).unwrap();

    let (width, height) = decoder.dimensions();
    let channel = match decoder.color_type() {
        image::ColorType::Rgba8 => 4,
        image::ColorType::Rgb8 => 3,
        _ => {
            // panic!("invalid color type")
            4
        }
    };

    let mut p: Vec<u8> = vec![0; decoder.total_bytes() as usize];
    decoder.read_image(&mut p).expect("webp decode error");

    Image {
        width,
        height,
        _pixels: vec![],
        channel,
    }
}
