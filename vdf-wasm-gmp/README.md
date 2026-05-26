# vdf-wasm-gmp

Node/Emscripten WASM build that links the GMP-backed VDF code against a GMP
static library compiled for `wasm32-unknown-emscripten`.

Build and smoke-test:

```sh
make node-gmp-test
```

Benchmark:

```sh
npm run bench --prefix examples/node-wasm-gmp
```

This target requires Emscripten (`emcc`, `emconfigure`, and `emmake`). The
Makefile downloads GMP 6.3.0 into `target/`, builds a static WASM GMP library,
then links the `vdf-wasm-gmp` crate against it.

Emscripten uses GMP's generic WASM build, so benchmark against your actual
discriminant size and difficulty.
