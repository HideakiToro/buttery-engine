Code in this project has so far only been based on:
1. https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera.
2. Copilot (primarily the depth buffer)

The code has been split between multiple files for easier extendability.

To build this project, this guide was used: https://github.com/gfx-rs/wgpu/wiki/Running-on-the-Web-with-WebGPU-and-WebGL

```bash
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --profile release
wasm-bindgen --out-dir target/generated --web target/wasm32-unknown-unknown/release/graphics-test.wasm
simple-http-server target/generated
```