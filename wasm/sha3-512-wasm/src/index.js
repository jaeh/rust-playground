const rust = import('../pkg/sha3_512_wasm')

const run = async () => {
  try {
    const m = await rust
    const world = await m.hash('world')
    console.log('m.hash("world") returned: ', world);
    const world2 = await m.hash('world')
    console.log('m.hash("world") returned: ', world2);

    console.log('equal hashes:', world === world2)

    const str = `
    Lorem Ipsum is simply dummy text of the printing and typesetting industry. 
    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, 
    when an unknown printer took a galley of type and scrambled it to make a type specimen book. 
    It has survived not only five centuries, but also the leap into electronic typesetting, 
    remaining essentially unchanged. 
    It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, 
    and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.`

    const str_hash = await m.hash(str)
    console.log('m.hash("str") returned: ', str_hash)

  //   const resultDiv = document.getElementById('resultworld')
  //   resultDiv.innerHTML = `
  //   1. m.hash("world") returned: ${world}
  //   <br /> 
  //   2. m.hash("world") returned: ${world2}
  //   <br />
  //   3. m.hash(${str}) returned: ${str_hash}  
  // ` 
  } catch (e) {
    console.error(e)
  }
}

run()
