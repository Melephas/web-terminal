cargo build --release --target wasm32-unknown-unknown
rm wasm/web-terminal{.js,_bg.wasm}
wasm-bindgen target/wasm32-unknown-unknown/release/web-terminal.wasm --out-dir wasm --no-modules --no-typescript
http-server wasm
