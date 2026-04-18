use super::camera_controller::CameraController;
use crate::core::{
    camera::Camera,
    engine::ButteryEngineState,
    game::ButteryGame,
    key_event::{Key, KeyEvent},
};
use cgmath::{Deg, Point3, Rad};
use std::f32::consts::PI;

pub struct ButteryExample {
    camera: Camera,
    camera_controller: CameraController,
}

impl ButteryExample {
    pub fn new() -> Self {
        let camera = Camera::new((0.0, 4.0, 6.0), Deg(-90.0), Deg(-35.0));

        Self {
            camera,
            camera_controller: CameraController::new(4.0, 0.4),
        }
    }
}

impl ButteryGame for ButteryExample {
    fn get_title(&self) -> String {
        "Butter-Engine Example".into()
    }

    fn on_init(&mut self, state: &mut ButteryEngineState) {
        state.renderer.load_model("./models/cube.glb");
    }

    fn on_key_event(&mut self, _state: &mut ButteryEngineState, key_event: KeyEvent) {
        match key_event.key {
            // Key::Escape => {
            //     if state.renderer.open_menu {
            //         self.open_menu = false;
            //     } else {
            //         event_loop.exit();
            //     }
            // }
            // (KeyCode::KeyE, true) if !self.open_menu => {
            //     self.open_menu = true;
            // }
            Key::R if key_event.pressed => {
                self.camera.yaw -= Rad(PI * 0.5);
            }
            Key::L if key_event.pressed => {
                self.camera.yaw = Rad(-PI * 0.5);
                self.camera.position = Point3::new(0.0, 4.0, 6.0);
            }
            _ => {
                self.camera_controller.handle_key_event(key_event);
            }
            _ => {}
        }
    }
}
