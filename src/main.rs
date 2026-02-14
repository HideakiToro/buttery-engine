use graphics_test::app::App;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

fn main() -> anyhow::Result<()> {
    run()
}

fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        match console_log::init_with_level(log::Level::Info) {
            Ok(_) => {}
            Err(e) => {
                println!("{e:#?}");
                return Ok(());
            }
        };
    }

    let event_loop = match EventLoop::with_user_event().build() {
        Ok(e) => e,
        Err(e) => {
            println!("{e:#?}");
            return Ok(());
        }
    };
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    match event_loop.run_app(&mut app) {
        Ok(_) => {}
        Err(e) => {
            println!("{e:#?}");
            return Ok(());
        }
    };

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
