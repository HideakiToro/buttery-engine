use super::renderer::SlipperyRenderer;
use crate::core::{
    engine::ButteryEngine,
    key_event::{Key as ButteryKey, KeyEvent as ButteryKeyEvent},
    renderer::ButteryRenderer,
    windowing::ButteryWindowingSystem,
};
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use winit::event_loop::EventLoopProxy;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub struct SlipperyRendererWindowing {}

impl SlipperyRendererWindowing {
    pub fn new() -> Box<dyn ButteryWindowingSystem> {
        Box::new(Self {}) as Box<dyn ButteryWindowingSystem>
    }
}

pub struct State {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<EventLoopProxy<SlipperyRenderer>>,
    pub engine: ButteryEngine,
}

impl ButteryWindowingSystem for SlipperyRendererWindowing {
    fn run(&self, engine: ButteryEngine) -> anyhow::Result<()> {
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

        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        let mut state = State {
            engine,
            #[cfg(target_arch = "wasm32")]
            proxy,
        };

        match event_loop.run_app(&mut state) {
            Ok(_) => {}
            Err(e) => {
                println!("{e:#?}");
                return Ok(());
            }
        };

        Ok(())
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

        window_attributes.title = self.engine.game.get_title();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the

            use crate::{
                core::renderer::ButteryRenderer, slippery_renderer::renderer::SlipperyRenderer,
            };
            self.engine.state.renderer =
                Box::new(pollster::block_on(SlipperyRenderer::new(window)).unwrap())
                    as Box<dyn ButteryRenderer>;
            self.engine.on_init();
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the
            // proxy to send the results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(
                                SlipperyRenderer::new(window)
                                    .await
                                    .expect("Unable to create canvas!!!")
                            )
                            .is_ok()
                    )
                });
            }
        }
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
        self.engine.state.renderer = Box::new(renderer) as Box<dyn ButteryRenderer>;
        #[cfg(target_arch = "wasm32")]
        self.engine.on_init();
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

        if let Some(slippery_renderer) = self
            .engine
            .state
            .renderer
            .as_any_mut()
            .downcast_mut::<SlipperyRenderer>()
        {
            let _ = slippery_renderer
                .egui_state
                .on_window_event(slippery_renderer.window.as_ref(), &event);
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.engine.state.renderer.resize(size.width, size.height)
            }
            WindowEvent::RedrawRequested => {
                self.engine.calc_delta_time();

                self.engine.on_update();

                self.engine.state.renderer.render();
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
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                self.engine.on_key_event(ButteryKeyEvent {
                    key: code.into(),
                    pressed: key_state.is_pressed(),
                });
            }
            _ => {}
        }
    }

    // ...
}

impl From<KeyCode> for ButteryKey {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::ArrowUp => Self::ArrowUp,
            KeyCode::ArrowDown => Self::ArrowDown,
            KeyCode::ArrowLeft => Self::ArrowLeft,
            KeyCode::ArrowRight => Self::ArrowRight,
            KeyCode::KeyA => Self::A,
            KeyCode::KeyB => Self::B,
            KeyCode::KeyC => Self::C,
            KeyCode::KeyD => Self::D,
            KeyCode::KeyE => Self::E,
            KeyCode::KeyF => Self::F,
            KeyCode::KeyG => Self::G,
            KeyCode::KeyH => Self::H,
            KeyCode::KeyI => Self::I,
            KeyCode::KeyJ => Self::J,
            KeyCode::KeyK => Self::K,
            KeyCode::KeyL => Self::L,
            KeyCode::KeyM => Self::M,
            KeyCode::KeyN => Self::N,
            KeyCode::KeyO => Self::O,
            KeyCode::KeyP => Self::P,
            KeyCode::KeyQ => Self::Q,
            KeyCode::KeyR => Self::R,
            KeyCode::KeyS => Self::S,
            KeyCode::KeyT => Self::T,
            KeyCode::KeyU => Self::U,
            KeyCode::KeyV => Self::V,
            KeyCode::KeyW => Self::W,
            KeyCode::KeyX => Self::X,
            KeyCode::KeyY => Self::Y,
            KeyCode::KeyZ => Self::Z,
            KeyCode::Digit0 => Self::Key0,
            KeyCode::Digit1 => Self::Key1,
            KeyCode::Digit2 => Self::Key2,
            KeyCode::Digit3 => Self::Key3,
            KeyCode::Digit4 => Self::Key4,
            KeyCode::Digit5 => Self::Key5,
            KeyCode::Digit6 => Self::Key6,
            KeyCode::Digit7 => Self::Key7,
            KeyCode::Digit8 => Self::Key8,
            KeyCode::Digit9 => Self::Key9,
            KeyCode::ControlLeft => Self::LeftCtrl,
            KeyCode::ControlRight => Self::RightCtrl,
            KeyCode::ShiftLeft => Self::LeftShift,
            KeyCode::ShiftRight => Self::RightShift,
            KeyCode::Enter => Self::Enter,
            KeyCode::Escape => Self::Escape,
            key => {
                println!("Unknown key {key:#?} event");
                Self::None
            }
        }
    }
}
