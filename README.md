# sort-table-wasm

DISCLAIMER: This code is probably slow and buggy.  It's not something I spent much time on.



Table sorting function for JS Gamify project compiled to WASM

## Building
1. Install rustup (https://rustup.rs/)
2. Run ``` cargo install wasm-pack wasm-bindgen-cli ```
3. Clone the repo, cd inside and run ``` cargo build --target wasm32-unknown-unknown --release ```
4. cd into target/wasm.../release/ and run ``` wasm-bindgen --target web --no-typescript --out-dir . sort_wasm.wasm ```
5. The "sort_wasm_bg.wasm" and "sort_wasm.js" will be ready to use

<br>



#### License



<sup>

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version

2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

</sup>



<br>



<sub>

Unless you explicitly state otherwise, any contribution intentionally submitted

for inclusion in sort-table-wasm by you, as defined in the Apache-2.0 license, shall be

dual licensed as above, without any additional terms or conditions.

</sub>



