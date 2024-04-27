# kotor2-wine-load-remover

An auto splitter for Star Wars: Knights of the Old Republic II: The Sith Lords

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain stable
```

The auto splitter can now be compiled:
```sh
cargo b --release
```

The auto splitter is then available at:
```
target/wasm32-unknown-unknown/release/kotor2_wine_load_remover.wasm
```

Make sure too look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

## Credit


Credit to Refragg for the [base of this project](https://github.com/Refragg/sadx-wasm-auto-splitter/)
