import { cli } from '@magic/cli/src/index.mjs'
import log from '@magic/log'

import { sha3 } from '../src/index.mjs'

const args = {
  options: [['--help', '-help', 'help', '--h', '-h'], ['--in', '-i'], ['--bits', '-b']],
  commands: ['sha3'],
  help: {
    name: 'keccak',
    header: 'wrap tiny-keccak rust crate with node bindings',
    options: {
      in: 'string to hash using sha3.',
      bits: '224, 256, 384 or 512',
    },
    commands: {
      sha3: 'run sha3 on --in flag arguments',
    },
    example: `
bin/bin.js echo -e testing a list of words

npm start -- testing a list of words

npm start -- "list of words with '&*special chars'"
`,
  },
}

const res = cli(args)
let nodeToRust = res.argv['--in'].join(' ')

if (!nodeToRust) {
  log.error('use:', 'npm start -- some words')
  process.exit()
}

log.info('sending string to rust:', nodeToRust)

try {
  const rustToNode = sha3(nodeToRust)
  log.success('success', 'received from rust:', rustToNode)
} catch(e) {
  log.error('error when hashing', e)
}
