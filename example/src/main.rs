use buttery_engine::engine::ButteryEngine;
use buttery_engine_example::core::ButteryExample;
use slippery_renderer::windowing::SlipperyRendererWindowing;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() -> anyhow::Result<()> {
    let game = ButteryExample::new();
    ButteryEngine::run(SlipperyRendererWindowing::new(), game)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    let game = ButteryExample::new();
    if let Err(e) = ButteryEngine::run(SlipperyRendererWindowing::new(), game) {
        println!("{e:#?}");
    }

    Ok(())
}
