use super::renderer::SlipperyRenderer;
use buttery_engine::{
    engine::ButteryEngine,
    game::ButteryGame,
    key_event::{Key as ButteryKey, KeyEvent as ButteryKeyEvent},
    renderer::ButteryRenderer,
    windowing::ButteryWindowingSystem,
};
use std::{marker::PhantomData, sync::Arc};
#[cfg(target_arch = "wasm32")]
use winit::event_loop::EventLoopProxy;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

pub struct SlipperyRendererWindowing<G: ButteryGame> {
    _phantom_data: PhantomData<G>,
}

impl<G: ButteryGame> SlipperyRendererWindowing<G> {
    pub fn new() -> Box<dyn ButteryWindowingSystem<G>> {
        Box::new(Self {
            _phantom_data: PhantomData {},
        }) as Box<dyn ButteryWindowingSystem<G>>
    }
}

pub struct State<G: ButteryGame> {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<EventLoopProxy<SlipperyRenderer<G>>>,
    pub engine: ButteryEngine<G>,
}

impl<G: ButteryGame> ButteryWindowingSystem<G> for SlipperyRendererWindowing<G> {
    fn run(&self, engine: ButteryEngine<G>) -> anyhow::Result<()> {
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

impl<G: ButteryGame> ApplicationHandler<SlipperyRenderer<G>> for State<G> {
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
            // await the initialization of the renderer
            self.engine.state.renderer =
                Box::new(pollster::block_on(SlipperyRenderer::new(window)).unwrap())
                    as Box<dyn ButteryRenderer<G>>;
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
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut renderer: SlipperyRenderer<G>) {
        // This is where proxy.send_event() ends up
        #[cfg(target_arch = "wasm32")]
        {
            renderer.window.request_redraw();
            renderer.resize(
                renderer.window.inner_size().width,
                renderer.window.inner_size().height,
            );
        }
        self.engine.state.renderer = Box::new(renderer) as Box<dyn ButteryRenderer<G>>;
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
            .downcast_mut::<SlipperyRenderer<G>>()
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

                self.engine.state.renderer.render(&mut self.engine.game);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                self.engine.on_key_event(ButteryKeyEvent {
                    key: key_code_to_buttery_key(key_code),
                    pressed: key_state.is_pressed(),
                });
            }
            _ => {}
        }
    }

    // ...
}

fn key_code_to_buttery_key(value: KeyCode) -> ButteryKey {
    match value {
        KeyCode::ArrowUp => ButteryKey::ArrowUp,
        KeyCode::ArrowDown => ButteryKey::ArrowDown,
        KeyCode::ArrowLeft => ButteryKey::ArrowLeft,
        KeyCode::ArrowRight => ButteryKey::ArrowRight,
        KeyCode::KeyA => ButteryKey::A,
        KeyCode::KeyB => ButteryKey::B,
        KeyCode::KeyC => ButteryKey::C,
        KeyCode::KeyD => ButteryKey::D,
        KeyCode::KeyE => ButteryKey::E,
        KeyCode::KeyF => ButteryKey::F,
        KeyCode::KeyG => ButteryKey::G,
        KeyCode::KeyH => ButteryKey::H,
        KeyCode::KeyI => ButteryKey::I,
        KeyCode::KeyJ => ButteryKey::J,
        KeyCode::KeyK => ButteryKey::K,
        KeyCode::KeyL => ButteryKey::L,
        KeyCode::KeyM => ButteryKey::M,
        KeyCode::KeyN => ButteryKey::N,
        KeyCode::KeyO => ButteryKey::O,
        KeyCode::KeyP => ButteryKey::P,
        KeyCode::KeyQ => ButteryKey::Q,
        KeyCode::KeyR => ButteryKey::R,
        KeyCode::KeyS => ButteryKey::S,
        KeyCode::KeyT => ButteryKey::T,
        KeyCode::KeyU => ButteryKey::U,
        KeyCode::KeyV => ButteryKey::V,
        KeyCode::KeyW => ButteryKey::W,
        KeyCode::KeyX => ButteryKey::X,
        KeyCode::KeyY => ButteryKey::Y,
        KeyCode::KeyZ => ButteryKey::Z,
        KeyCode::Digit0 => ButteryKey::Key0,
        KeyCode::Digit1 => ButteryKey::Key1,
        KeyCode::Digit2 => ButteryKey::Key2,
        KeyCode::Digit3 => ButteryKey::Key3,
        KeyCode::Digit4 => ButteryKey::Key4,
        KeyCode::Digit5 => ButteryKey::Key5,
        KeyCode::Digit6 => ButteryKey::Key6,
        KeyCode::Digit7 => ButteryKey::Key7,
        KeyCode::Digit8 => ButteryKey::Key8,
        KeyCode::Digit9 => ButteryKey::Key9,
        KeyCode::ControlLeft => ButteryKey::LeftCtrl,
        KeyCode::ControlRight => ButteryKey::RightCtrl,
        KeyCode::ShiftLeft => ButteryKey::LeftShift,
        KeyCode::ShiftRight => ButteryKey::RightShift,
        KeyCode::Enter => ButteryKey::Enter,
        KeyCode::Escape => ButteryKey::Escape,
        KeyCode::AltLeft => ButteryKey::LeftAlt,
        KeyCode::AltRight => ButteryKey::RightAlt,
        KeyCode::Backspace => ButteryKey::Backspace,
        KeyCode::CapsLock => ButteryKey::CapsLock,
        KeyCode::Comma => ButteryKey::Comma,
        KeyCode::Minus => ButteryKey::Minus,
        KeyCode::Period => ButteryKey::Period,
        KeyCode::Space => ButteryKey::Space,
        KeyCode::SuperLeft => ButteryKey::OSLeft,
        KeyCode::SuperRight => ButteryKey::OSRight,
        KeyCode::Tab => ButteryKey::Tab,
        key => {
            println!("Unknown key {key:#?} event");
            ButteryKey::None
        }
    }
}
