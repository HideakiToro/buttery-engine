use buttery_engine::{
    core::engine::ButteryEngine, example::core::ButteryExample,
    slippery_renderer::windowing::SlipperyRendererWindowing,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() -> anyhow::Result<()> {
    let game = Box::new(ButteryExample::new());
    ButteryEngine::run(SlipperyRendererWindowing::new(), game);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    ButteryEngine::run(SlipperyRendererWindowing::new());

    Ok(())
}
