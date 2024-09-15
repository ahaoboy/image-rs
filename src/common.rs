use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Image {
    pub(crate) _pixels: Vec<u8>,
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
