use wasm_bindgen::prelude::*;

use sha3::{Digest, Sha3_512};

#[wasm_bindgen]
pub fn hash(val: &str) -> String {
  // create a SHA3-512 object
  let mut hasher = Sha3_512::new();

  // write input message
  hasher.input(val);

  // read hash digest
  let result = hasher.result();

  format!("{:x}", result)
}

#[test]
fn test_ohai() {
    let result = hash("hello world");
    let expected = hash("hello world");

    assert_eq!(result.len(), expected.len());
}
