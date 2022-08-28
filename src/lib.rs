use wasm_bindgen::prelude::*; // JS与Rust交互的包

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(name);
}

