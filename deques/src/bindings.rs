use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log_str(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn console_log_str(s: &str) {
    println!("{}", s);
}

macro_rules! log {
    ($($t:tt)*) => (console_log_str(&format_args!($($t)*).to_string()))
}
pub(crate) use log; // make log macro public

#[wasm_bindgen]
pub struct TTAFPlayer {}

#[wasm_bindgen]
impl TTAFPlayer {
    pub fn new() -> Self {
        Self {}
    }
}


// Coming soon!
