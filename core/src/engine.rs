use std::time::Duration;
use web_time::Instant;

use crate::{
    game::ButteryGame,
    key_event::KeyEvent,
    object::Object,
    registry::Registry,
    renderer::{ButteryRenderer, FallbackRenderer},
    windowing::ButteryWindowingSystem,
    world_model::ButteryWorldModel,
};

pub enum ButteryEvent {
    Init,
    Update,
    KeyPress(String),
}

pub struct ButteryEngineState {
    pub renderer: Box<dyn ButteryRenderer>,
    last_frame_time: Instant,
    pub delta_time: f32,
    pub world_model: ButteryWorldModel,
    pub world_diff: Registry<Object>,
}

pub struct ButteryEngine {
    pub state: ButteryEngineState,
    pub game: Box<dyn ButteryGame>,
}

impl ButteryEngine {
    pub fn run(
        windowing_system: Box<dyn ButteryWindowingSystem>,
        game: Box<dyn ButteryGame>,
    ) -> anyhow::Result<()> {
        let engine = Self {
            game,
            state: ButteryEngineState {
                renderer: Box::new(FallbackRenderer {}),
                delta_time: 1.0 / 60.0,
                last_frame_time: web_time::Instant::now(),
                world_model: ButteryWorldModel::default(),
                world_diff: Registry::default(),
            },
        };

        windowing_system.run(engine)
    }

    pub fn on_init(&mut self) {
        self.game.on_init(&mut self.state);
    }

    pub fn on_update(&mut self) {
        self.state.world_diff.reset();

        for (_, object) in self.state.world_model.objects.iter_mut() {
            object.on_update(&mut self.state.world_diff, self.state.delta_time);
        }

        self.game.on_update(&mut self.state);

        self.state
            .world_model
            .apply_diff(&mut self.state.world_diff);

        self.state.renderer.on_update(&self.state.world_model);
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
