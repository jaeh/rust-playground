use neon::prelude::*;
use neon::register_module;

fn echo(mut cx: FunctionContext) -> JsResult<JsString> {
    let string_to_hash = cx.argument::<JsString>(0)?.value();

    if string_to_hash.len() == 0 {
        panic!("Received empty string");
    }

    println!("string in rust: {}", string_to_hash);

    Ok(cx.string(string_to_hash))
}

register_module!(mut m, {
    m.export_function("echo", echo)?;
    Ok(())
});
