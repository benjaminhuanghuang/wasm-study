Setup
```
  cargo install wasm-pack

  cargo new --lib hello-wasm


  # build
  wasm-pack build --target web
```

add
```
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```
to Cargo.toml
