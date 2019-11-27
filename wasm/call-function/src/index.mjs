import init, { greet } from '/pkg/call_function.js'

/**
 * This function gets called from rust
 *
 * @param {String} response returned from rust 
 */
const respond = response => {
	console.log('rust called respond in src/index.mjs. arguments:', response)
	document.body.innerHTML += `<p>rust says: <i>${response}</i></p>`
}

globalThis.respond = respond

/**
 * Async/Awaits our return from rust.
 * We are in async land since we do not know how long rust computes.
 * 
 * @since 0.0.1
 */
const run = async () => {
  try {
		await init("/pkg/call_function_bg.wasm")
	
    // no need to await, globalThis.respond gets called as callback
    // this is still async, but does not even add a promise.
    // keeping this in the try catch is a good idea though.
    // usually this also is unwanted.
    // the only reason to do this is if you can not avoid passing values
    // from rust to javascript and back.
    greet('world')
  } catch (e) {
    console.error(e)
  }
}

run()
