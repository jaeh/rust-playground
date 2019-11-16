use neon::prelude::*;
use neon::register_module;

use tiny_keccak::{Hasher, Sha3};

fn sha3(mut cx: FunctionContext) -> JsResult<JsArray> {
    let bits = match cx.argument_opt(0) {
        Some(arg) => arg.downcast::<JsNumber>().or_throw(&mut cx)?.value(),
        // Default to 12 if no value is given
        None => 256 as f64,
    };

    let string_to_hash = cx.argument::<JsString>(1)?.value();

    if string_to_hash.len() == 0 {
        panic!("Received empty string");
    }

    let mut sha3_hasher;
    if bits == 224 as f64 {
        sha3_hasher = Sha3::v224();
    } else if bits == 384 as f64 {
        sha3_hasher = Sha3::v384();
    } else if bits == 512 as f64 {
        sha3_hasher = Sha3::v512();
    } else {
        sha3_hasher = Sha3::v256();
    }

    let mut output = [0u8; 32];

    sha3_hasher.update(string_to_hash.as_bytes());
    sha3_hasher.finalize(&mut output);

    let js_array = JsArray::new(&mut cx, output.len() as u32);

    // // Iterate over the rust Vec and map each value in the Vec to the JS array
    for (i, obj) in output.iter().enumerate() {
        let string  = cx.string(format!("{:x}", *obj));
        js_array.set(&mut cx, i as u32, string).unwrap();
    }

    Ok(js_array)
}

register_module!(mut m, {
    m.export_function("sha3", sha3)?;
    Ok(())
});
