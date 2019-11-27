import init, { hash } from '/pkg/sha3_512_wasm.js'

import nodeHashes from '/pkg/node-hashes.mjs'

const run = async () => {
  const { body } = document

  const { sha3_512 } = window

  const hashes = {
    node: nodeHashes,
    browser: {},
    rust: {},
  }

  await init("/pkg/sha3_512_wasm_bg.wasm");

  const str1 = "Hail Eris!"
  const str2 = "All hail discordia!"

  const str1Hash1 = hash(str1)

  const str1Hash2 = hash(str1)

  const str2Hash = hash(str2)

  const lorem = `
    Lorem Ipsum is simply dummy text of the printing and typesetting industry. 
    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, 
    when an unknown printer took a galley of type and scrambled it to make a type specimen book. 
    It has survived not only five centuries, but also the leap into electronic typesetting, 
    remaining essentially unchanged. 
    It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, 
    and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.`

  const loremHash = hash(lorem)
  const loremHash2 = hash(lorem)
  
  const browser = {
    loremHash: sha3_512(lorem),
    str1Hash: sha3_512(str1),
    str2Hash: sha3_512(str2),
  }

  body.innerHTML += `
<h1>Hashing in rust</h1>

<h3>input: "${str1}"</h3>

<p>hash #1: "${str1Hash1}"</p>
<p>hash #2: "${str1Hash2}"</p>
<h3>input: "${str1}"</h3>
<p>rust:    "${str2Hash}"</p>

<h3>Lorem Ipsum:</h3>

<p>input: "${lorem}"</p>

<p>hash #1:  "${loremHash}"</p>

<p>hash #2: "${loremHash2}"</p>

<h1>Equality </h1>

<p>${str1Hash1 === str1Hash2}, both rust hashes for "${str1}" are equal</p>

body.innerHTML += '<h4>test output of nodejs, browser and rust for equality</h4>'

<p>${str1Hash1 === browser.str1Hash} hashes for "${str1}" are equal in rust and browser js</p>
<p>${str1Hash1 === nodeHashes.str1Hash} hashes for "${str1}" are equal in rust and node js</p>

<p>${str2Hash === browser.str2Hash} hashes for "${str2}" are equal in rust and browser js</p>
<p>${str2Hash === nodeHashes.str2Hash} hashes for "${str2}" are equal in rust and node js</p>

<p>${loremHash === browser.loremHash} hashes for "lorem ipsum..." are equal in rust and browser js</p>
<p>${loremHash === nodeHashes.loremHash} hashes for "lorem ipsum..." are equal in rust and node js</p>
`
}

run()