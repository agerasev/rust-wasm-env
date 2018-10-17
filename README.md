# WASM Env

Simple environment for running Rust WASM applications in Web-browser

## Sample applications

+ [Gravity game](https://github.com/nthend/rust-wasm-gravity.git) ([try it!](https://nthend.github.io/rust-wasm-gravity))

## Documentation

+ [Master branch documentation](https://nthend.github.io/rust-wasm-env/target/doc/wasm_env/)

## Usage


Create cargo lib project:

```sh
mkdir "rust-wasm-proj"
cd "rust-wasm-proj"
cargo init --lib
git init
```


Add `rust-wasm-env` as a submodule to this project:

```sh
git submodule add https://github.com/nthend/rust-wasm-env.git wasm
```


Add these lines to your project's `Cargo.toml`:

```toml
[lib]
name = "main"
crate-type = ["cdylib"]

[dependencies]
lazy_static = "1.1.0"
wasm_env = { path = "wasm" }
```


Put the following code into your `src/lib.rs`:

```rust
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate wasm_env as wasm;

struct App {}

impl App {
	fn new() -> Self {
		wasm::console::log("Hello, WASM!");
		App {}
	}
}

impl wasm::App for App {
    fn handle(&mut self, event: wasm::Event) {
        wasm::console::log(&format!("{:?}", event));
    }
}

wasm_bind!(wasm, || Box::new(App::new()));
```

To run following scripts you need to have [Python 3](https://www.python.org/download/releases/3.0/)) installed.

To build your project:

```sh
python3 wasm/script/build.py
```

To run your project you need a simple file server:

```sh
python3 wasm/script/server.py
```



Now open your browser [http://localhost:8000](http://localhost:8000) and open browser console.
You will see `Hello World!` message and a lot of event messages.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
