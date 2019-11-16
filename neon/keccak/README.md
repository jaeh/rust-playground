## echo

pass a string from js to rust 
and echo it back as a string to node.

### usage
```bash
# node dependencies (tests and logging)
npm install

# first build the rust native code
npm run build

# run the lib via cli in bin
npm start -- your string to echo

npm start -- "your string with 'special chars &'"

bin/bin.js echo -e your string

```
