cargo build --release --target=wasm32-unknown-unknown
cp "./target/wasm32-unknown-unknown/release/main.wasm" "./main-big.wasm"
wasm-gc "./main-big.wasm" "./main.wasm"
rm "./main-big.wasm"
