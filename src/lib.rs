mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::str::FromStr;

use chunk::Chunk;
use chunk_type::ChunkType;
use js_sys::Uint8Array;
use png::Png;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn encode(
    buffer: &JsValue,
    chunk_type: &str,
    message: &str,
) -> std::result::Result<JsValue, String> {
    let mut byte_array = Uint8Array::new(buffer).to_vec();
    byte_array.append(
        &mut Chunk::new(
            match ChunkType::from_str(chunk_type) {
                Ok(x) => x,
                Err(_) => return Err("incompatible chunk type".into()),
            },
            message.as_bytes().try_into().unwrap(),
        )
        .as_bytes(),
    );

    let byte_array: &[u8] = &byte_array;

    Ok(Uint8Array::from(byte_array).into())
}

#[wasm_bindgen]
pub fn remove(buffer: &JsValue, chunk_type: &str) -> std::result::Result<JsValue, String> {
    let byte_array: &[u8] = &Uint8Array::new(buffer).to_vec();

    let mut png = Png::try_from(byte_array).unwrap();

    match png.remove_chunk(chunk_type) {
        Ok(_) => {
            let byte_array: &[u8] = &png.as_bytes();
            Ok(Uint8Array::from(byte_array).into())
        }
        Err(x) => return Err(x.to_string()),
    }
}

#[wasm_bindgen]
pub fn decode(buffer: &JsValue, chunk_type: &str) -> std::result::Result<String, String> {
    let byte_array: &[u8] = &Uint8Array::new(buffer).to_vec();

    match Png::try_from(byte_array).unwrap().chunk_by_type(chunk_type) {
        Some(x) => Ok(x.data_as_string().unwrap()),
        None => return Err("no such chunk".into()),
    }
}

#[wasm_bindgen]
pub fn check_for_chunk(buffer: &JsValue, chunk_type: &str) -> bool {
    set_panic_hook();

    let byte_array: &[u8] = &Uint8Array::new(buffer).to_vec();

    match Png::try_from(byte_array).unwrap().chunk_by_type(chunk_type) {
        Some(_) => true,
        None => false,
    }
}
