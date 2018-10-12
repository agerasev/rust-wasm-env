cargo build --target=wasm32-unknown-unknown
copy ".\target\wasm32-unknown-unknown\debug\main.wasm" ".\main.wasm"
copy ".\wasm\html\index.html" ".\index.html"
