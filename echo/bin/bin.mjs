import { cli } from '@magic/cli/src/index.mjs'
import log from '@magic/log'

import { echo } from '../src/index.mjs'

const args = {
  options: [['--help', '-help', 'help', '--h', '-h'], ['--echo', '-e']],
  env: [[['--production', '--prod', '--p', '-p'], 'NODE_ENV', 'production']],
  commands: [['echo', 'e']],
  help: {
    name: 'magic',
    header: 'static and serverless page generator',
    options: {
      echo: 'string to echo through rust.',
    },
    commands: {
      echo: 'initiate echo command',
    },
    example: `
bin/bin.js echo -e testing a list of words

npm start -- testing a list of words

npm start -- "list of words with '&*special chars'"
`,
  },
}

const res = cli(args)
let nodeToRust = res.argv['--echo'].join(' ')

if (!nodeToRust) {
  nodeToRust = 'use: npm start -- some words'
}

log.info('sending string to rust:', nodeToRust)

try {
  const rustToNode = echo(nodeToRust)
  log.success('success', 'received string from rust:', rustToNode)
} catch(e) {
  log.error('error when echoing', e)
}

