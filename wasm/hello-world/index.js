// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import('./pkg/hello_world');

// es5 promises
// rust
//   .then(m => m.greet('World!'))
//   .catch(console.error);

// es6+ async/await
const run = async () => {
  try {
    const m = await rust
    const world = await m.greet('world')
    console.log('m.greet("world") returned: ', world)

    const moon = await m.greet('moon')
    console.log('m.greet("moon") returned: ', moon)
  } catch (e) {
    console.error(e)
  }
}

run()
