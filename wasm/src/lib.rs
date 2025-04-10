use hangul::Hangul;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = disassemble)]
pub fn disassemble(text: &str) -> String {
  Hangul::new(text).disassemble()
}

#[wasm_bindgen(js_name = getChoseong)]
pub fn get_choseong(text: &str) -> String {
  Hangul::new(text).get_choseong()
}
