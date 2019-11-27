use sha3::{Digest, Sha3_512};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash(val: &str) -> JsValue {
    // create a SHA3-512 object
    let mut hasher = Sha3_512::new();

    // write input message
    hasher.input(val);

    // read hash digest
    let result = hasher.result();

    format!("{:x}", result).into()
}
