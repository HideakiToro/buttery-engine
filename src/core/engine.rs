use std::time::{Duration, Instant};

use crate::core::{object::Object, renderer::{ButteryRenderer, FallbackRenderer}, windowing::ButteryWindowingSystem};

pub enum ButteryEvent {
    Init,
    Update,
    KeyPress(String),
}

pub struct ButteryEngine {
    pub renderer: Box<dyn ButteryRenderer>,
    objects: Vec<Object>,
    last_frame_time: Instant,
    delta_time: f32
}

impl ButteryEngine {
    pub fn start(windowing_system: Box<dyn ButteryWindowingSystem>) {
        let engine = Self {
            renderer: Box::new(FallbackRenderer {}),
            objects: vec![],
            delta_time: 1.0 / 60.0,
            last_frame_time: web_time::Instant::now()
        };

        windowing_system.run(engine);
    }

    pub fn on_init(&mut self) {
        self.renderer.load_model("./models/cube.glb");
    }

    pub fn on_update(&mut self) {
        for object in self.objects.iter_mut() {
            object.update();
        }

        self.renderer.on_update(&self.objects, self.delta_time);
    }

    pub fn on_keypress(&mut self) {
    }

    pub fn calc_delta_time(&mut self) {
        let now = web_time::Instant::now();
        let delta = now
            .checked_duration_since(self.last_frame_time)
            .unwrap_or(Duration::from_millis(16))
            .as_secs_f32();

        self.last_frame_time = now;
        self.delta_time = delta;
    }
}