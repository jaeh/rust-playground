import init, { greet } from '/pkg/hello_world.js';

const run = async () => {
  try {
    await init("/pkg/hello_world_bg.wasm");

    const { body } = document

    const world = await greet('hello, world')
    body.innerHTML = `${body.innerHTML}<p>m.greet("hello, world") returned: ${world}</p>`

    const moon = await greet('hello, moon')
    body.innerHTML = `${body.innerHTML}<p>m.greet("hello, moon") returned: ${moon}</p>`
  } catch (e) {
    console.error(e)
  }
}

run()
