# sort-table-wasm
Table sorting function for JS Gamify project compiled to WASM

## Building
1. Install rustup (https://rustup.rs/)
2. Run ``` cargo install wasm-pack wasm-bindgen-cli ```
3. Clone the repo, cd inside and run ``` cargo build --target wasm32-unknown-unknown --release ```
4. cd into target/wasm.../release/ and run ``` wasm-bindgen --target web --no-typescript --out-dir . sort_wasm.wasm ```
5. The "sort_wasm_bg.wasm" and "sort_wasm.js" will be ready to use
