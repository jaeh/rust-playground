const fs = require('fs')
const path = require('path')

const createKeccakHash = require('keccak')

const hash = toHash => createKeccakHash('sha3-512').update(toHash).digest('hex')

const testingString = "Hail Eris!"
const testingString2 = "All hail discordia!"

const loremIpsum = `
	Lorem Ipsum is simply dummy text of the printing and typesetting industry. 
	Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, 
	when an unknown printer took a galley of type and scrambled it to make a type specimen book. 
	It has survived not only five centuries, but also the leap into electronic typesetting, 
	remaining essentially unchanged. 
	It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, 
	and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.`

const testingStringHash = hash(testingString)
const testingStringHash2 = hash(testingString2)
const loremIpsumHash = hash(loremIpsum)


const content = `
export default {
	testingStringHash: '${testingStringHash}',
	testingStringHash2: '${testingStringHash2}',
	loremIpsum: '${loremIpsumHash}',
}
`

const file = path.join(process.cwd(), 'pkg', 'node-hashes.mjs')

fs.writeFileSync(file, content)