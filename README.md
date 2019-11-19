## Rust Playground

just a personal playground.

#### install
```bash
git clone https://github.com/jaeh/rust-playground
cd rust-playground
```

#### try a neon example
```bash
cd ./neon

# install dependencies
npm install

# move to example directory
cd ./echo

# if the package has js dependencies 
# (will warn you that nodejs packages are missing)
# npm install

npm run build
npm start
```

#### try a wasm example
```bash
cd ./wasm

# install dependencies
npm install

# move to example directory
cd ./hello-world

# if the package has js dependencies 
# (will warn you that nodejs packages are missing)
# npm install

npm run build
npm start

# wasm bundle now serves at localhost:8080
```

#### try a book example
```bash
cd ./book/1/hello-world

cargo run
```