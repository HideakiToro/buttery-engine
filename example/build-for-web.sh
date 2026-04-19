RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown --profile release
wasm-bindgen --out-dir target/generated --web target/wasm32-unknown-unknown/release/buttery-engine-example.wasm
cp ./index.html ./target/generated/index.html
cp -R ./src/models ./target/generated
simple-http-server target/generated