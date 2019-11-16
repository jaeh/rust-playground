import addon from '../native/index.node'

export const sha3 = (input, bits = 256) => {
	if (!input) {
		throw new Error('no input received')
	}

	if (typeof input !== 'string') {
		throw new Error('input has to be a string')
	}

	return addon.sha3(bits, input)
}

export default sha3
