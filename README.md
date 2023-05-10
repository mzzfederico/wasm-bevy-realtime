# Example with WASM + Bevy somewhat in real time.

A system in Bevy polls a bunch of time per frame new info from the JavaScript side. It gets rendered and updated as soon as it is on the Rust side. Just mocking things up.

## Building

```bash
cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/wasm-world-view.wasm
```

## Having a look

From the `out` folder:

```bash
npx serve .
```

Copy `assets` to `out` too.

## Notes

The spinning earth, materials and shading come from [this repo](https://github.com/nicopap/bevy_mod_paramap/blob/f2ecc42b79e9eb469485d79c7f2357fdf21f9fee/examples/earth3d.rs).

