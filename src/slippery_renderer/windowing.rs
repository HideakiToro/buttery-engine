use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use winit::event_loop::EventLoop;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::Window,
};
use crate::core::{engine::ButteryEngine, renderer::ButteryRenderer, windowing::ButteryWindowingSystem};
use super::renderer::SlipperyRenderer;

pub struct SlipperyRendererWindowing {
}

impl SlipperyRendererWindowing {
    pub fn new() -> Box<dyn ButteryWindowingSystem> {
        Box::new(Self {}) as Box<dyn ButteryWindowingSystem>
    }
}

pub struct State {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<Box<dyn ButteryRenderer>>>,
    pub engine: ButteryEngine
}

impl ButteryWindowingSystem for SlipperyRendererWindowing {
    fn run(&self, engine: ButteryEngine) {
        let mut state = State {
            engine,
            #[cfg(target_arch = "wasm32")]
            proxy: None,
        };

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
                return;
            }
        };

        #[cfg(target_arch = "wasm32")]
        app.set_event_loop(&event_loop);

        match event_loop.run_app(&mut state) {
            Ok(_) => {}
            Err(e) => {
                println!("{e:#?}");
            }
        };
    }
}

impl ApplicationHandler<SlipperyRenderer> for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        window_attributes.title = "Buttery-Engine".to_string();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the

            use crate::{core::renderer::ButteryRenderer, slippery_renderer::renderer::SlipperyRenderer};
            self.engine.renderer = Box::new(pollster::block_on(SlipperyRenderer::new(window)).unwrap()) as Box<dyn ButteryRenderer>;
        }

        // TODO: Fix init to use new systems
        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the
            // proxy to send the results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(
                                State::new(window)
                                    .await
                                    .expect("Unable to create canvas!!!")
                            )
                            .is_ok()
                    )
                });
            }
        }

        self.engine.on_init();
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut renderer: SlipperyRenderer) {
        // This is where proxy.send_event() ends up
        #[cfg(target_arch = "wasm32")]
        {
            renderer.window.request_redraw();
            renderer.resize(
                renderer.window.inner_size().width,
                renderer.window.inner_size().height,
            );
        }
        self.engine.renderer = Box::new(renderer) as Box<dyn ButteryRenderer>;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if event_loop.exiting() {
            return;
        }

        // let renderer = match &mut self.engine.renderer {
        //     Some(canvas) => canvas,
        //     None => return,
        // };

        // TODO: replace with 'self.engine.renderer.ui_event()'
        // let _ = self.engine.renderer
        //     .egui_state
        //     .on_window_event(state.window.as_ref(), &event);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => self.engine.renderer.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                self.engine.calc_delta_time();

                self.engine.on_update();

                self.engine.renderer.render();
                // match self.engine.renderer.render() {
                //     Ok(_) => {}
                //     // Reconfigure the surface if it's lost or outdated
                //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                //         // let size = state.window.inner_size();
                //         let size = self.engine.renderer.window_inner_size();
                //         self.engine.renderer.resize(size.width, size.height);
                //     }
                //     Err(e) => {
                //         log::error!("Unable to render {}", e);
                //     }
                // }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(_code),
                        state: _key_state,
                        ..
                    },
                ..
            } => {
                // self.engine.on_keypress(event_loop, code, key_state.is_pressed())
                self.engine.on_keypress()
            },
            _ => {}
        }
    }

    // ...
}
