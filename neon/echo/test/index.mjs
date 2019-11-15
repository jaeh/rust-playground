import { echo } from '../src/index.mjs'

export default [{ fn: echo('testing'), expect: 'testing', info: 'echo echos a value' }]
