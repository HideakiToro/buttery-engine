use buttery_engine::{core::engine::ButteryEngine, slippery_renderer::windowing::SlipperyRendererWindowing};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() -> anyhow::Result<()> {
    ButteryEngine::start(SlipperyRendererWindowing::new());
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    ButteryEngine::start();

    Ok(())
}
