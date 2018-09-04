# WASM Env

Simple environment for running WASM

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
    pub fn new() -> Self {
        wasm::console::log("Hello World!");
        App {}
    }
}

impl wasm::App for App {
    fn timeout(&mut self, _dt: f64) {}
    fn step(&mut self, _dt: f64) {}
    fn render(&mut self) {}
}

bind_wasm!(App, wasm);
```


To build your project run the following script:

```sh
./wasm/script/build_wasm.sh
```

Each `.sh` script has respectively named `.bat` script for running in Windows environment.


To run your project you need a simple file server (requires [Python 3](https://www.python.org/download/releases/3.0/)):

```sh
./wasm/script/run_server.sh
```

Now open your browser [http://localhost:8000/wasm/html](http://localhost:8000/wasm/html) and open browser console.
You will see `Hello World!` message.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
