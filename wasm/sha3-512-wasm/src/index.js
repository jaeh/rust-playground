import init, { hash } from '/pkg/sha3_512_wasm.js';

const run = async () => {
  await init("/pkg/sha3_512_wasm_bg.wasm");
  
  const testingString = "Hail Eris!"
  const testingString2 = "All hail discordia!"

  const hashedTesting = hash(testingString)
  console.log(`"${testingString}", hashed for the first time: "${hashedTesting}"`)

  const hashedTesting2 = hash(testingString)
  console.log(`${testingString}, hashed again, ${hashedTesting2}`)

  console.log(hashedTesting === hashedTesting2, `hashes for "${testingString}" are equal`)

  const testingString2Hash = hash(testingString2)
  console.log(`hashed "${testingString2}", ${testingString2Hash}`)

  const testingString2Hash2 = hash(testingString2)
  console.log(`hashed "${testingString2}" again, ${testingString2Hash2}`)

  console.log(testingString2Hash === testingString2Hash2, `hashes for ${testingString2} are equal`)

  const loremIpsum = `
    Lorem Ipsum is simply dummy text of the printing and typesetting industry. 
    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, 
    when an unknown printer took a galley of type and scrambled it to make a type specimen book. 
    It has survived not only five centuries, but also the leap into electronic typesetting, 
    remaining essentially unchanged. 
    It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, 
    and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.`

  const loremIpsumHash = await hash(loremIpsum)
  console.log('hash("lorem ipsum....") returned: ', loremIpsumHash )
}

run()