cargo build --target=wasm32-unknown-unknown
copy ".\target\wasm32-unknown-unknown\debug\wasm_test.wasm" ".\main.wasm"
