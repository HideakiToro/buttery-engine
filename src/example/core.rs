use super::camera_controller::CameraController;
use crate::core::{
    camera::Camera,
    engine::ButteryEngineState,
    game::ButteryGame,
    key_event::{Key, KeyEvent},
    ui::ButteryUIModel,
};
use cgmath::{Deg, Point3, Rad};
use std::f32::consts::PI;

pub struct ButteryExample {
    camera: Camera,
    camera_controller: CameraController,
    open_menu: bool,
}

impl ButteryExample {
    pub fn new() -> Self {
        let camera = Camera::new((0.0, 4.0, 6.0), Deg(-90.0), Deg(-35.0));

        Self {
            camera,
            camera_controller: CameraController::new(4.0, 0.4),
            open_menu: false,
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

    fn on_update(&mut self, state: &mut ButteryEngineState) {
        self.camera_controller
            .update_camera(&mut self.camera, state.delta_time);

        state.world_model.camera = self.camera;
    }

    fn on_key_event(&mut self, state: &mut ButteryEngineState, key_event: KeyEvent) {
        match key_event.key {
            Key::Escape if key_event.pressed => {
                if self.open_menu {
                    self.open_menu = false;
                    state.renderer.update_ui_model(None);
                }
            }
            Key::E if key_event.pressed && !self.open_menu => {
                self.open_menu = true;
                state.renderer.update_ui_model(Some(ButteryUIModel {}));
            }
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
        }
    }
}
