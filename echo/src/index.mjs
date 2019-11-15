import addon from '../native/index.node'

export const echo = input => addon.echo(input)

export default echo
