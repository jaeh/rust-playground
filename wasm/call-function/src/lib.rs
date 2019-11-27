use wasm_bindgen::prelude::*;

/// This function is defined in src/index.mjs and gets called from rust
#[wasm_bindgen]
extern "C" {
	fn respond(s: String);
}

/// This function gets called in src/index.mjs, 
/// and, in return, 
/// calls the respond function in the src/index.mjs file
#[wasm_bindgen]
pub fn greet(name: &str) {
	respond(format!("Hello, {}!", name));
}
