cargo build --target=wasm32-unknown-unknown
cp "./target/wasm32-unknown-unknown/debug/main.wasm" "./main.wasm"
cp "./wasm/html/index.html" "./index.html"
