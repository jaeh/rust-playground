use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[test]
fn test_ohai() {
    let result = greet("ohai");
    assert_eq!("Hello, ohai!", result);
}

#[test]
fn test_world() {
    let result = greet("World");
    assert_eq!("Hello, World!", result);
}
