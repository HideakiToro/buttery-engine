use std::time::{Duration, Instant};

use crate::core::{
    game::ButteryGame,
    key_event::KeyEvent,
    object::Object,
    renderer::{ButteryRenderer, FallbackRenderer},
    windowing::ButteryWindowingSystem,
};

pub enum ButteryEvent {
    Init,
    Update,
    KeyPress(String),
}

pub struct ButteryEngineState {
    pub renderer: Box<dyn ButteryRenderer>,
    // TODO: Replace with ButteryWorldModel Struct
    objects: Vec<Object>,
    last_frame_time: Instant,
    delta_time: f32,
}

pub struct ButteryEngine {
    pub state: ButteryEngineState,
    pub game: Box<dyn ButteryGame>,
}

impl ButteryEngine {
    pub fn run(windowing_system: Box<dyn ButteryWindowingSystem>, game: Box<dyn ButteryGame>) {
        let engine = Self {
            game,
            state: ButteryEngineState {
                renderer: Box::new(FallbackRenderer {}),
                objects: vec![],
                delta_time: 1.0 / 60.0,
                last_frame_time: web_time::Instant::now(),
            },
        };

        windowing_system.run(engine);
    }

    pub fn on_init(&mut self) {
        self.game.on_init(&mut self.state);
    }

    pub fn on_update(&mut self) {
        for object in self.state.objects.iter_mut() {
            object.update();
        }

        self.state
            .renderer
            .on_update(&self.state.objects, self.state.delta_time);
    }

    pub fn on_key_event(&mut self, key_event: KeyEvent) {
        self.game.on_key_event(&mut self.state, key_event);
    }

    pub fn calc_delta_time(&mut self) {
        let now = web_time::Instant::now();
        let delta = now
            .checked_duration_since(self.state.last_frame_time)
            .unwrap_or(Duration::from_millis(16))
            .as_secs_f32();

        self.state.last_frame_time = now;
        self.state.delta_time = delta;
    }
}
