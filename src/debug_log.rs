/*
pub fn log(msg: &str) {
    info!("{}", msg);
}
*/

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! wasm_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
}
