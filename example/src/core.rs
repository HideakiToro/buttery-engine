use crate::component::ExampleComponent;

use super::camera_controller::CameraController;
use buttery_engine::{
    camera::Camera,
    component::ButteryComponent,
    engine::ButteryEngineState,
    game::ButteryGame,
    key_event::{Key, KeyEvent},
    object::Object,
    ui::{ButteryUIElement, ButteryUIModel, ButteryUIWindow},
};
use cgmath::{Deg, Point3, Rad};
use std::f32::consts::PI;

pub struct ButteryExample {
    camera: Camera,
    light: Camera,
    camera_controller: CameraController,
    open_menu: bool,
    fps_text: String,
    frame_counter: i32,
    time_since_last_update: f32,
}

impl ButteryExample {
    pub fn new() -> Self {
        let camera = Camera::new((0.0, 4.0, 6.0), Deg(-90.0), Deg(-35.0));
        let light = Camera::new((30.0, 28.0, 0.0), Deg(-180.0), Deg(-35.0));

        Self {
            camera,
            light,
            camera_controller: CameraController::new(4.0, 0.4),
            open_menu: false,
            fps_text: "Hello World!".into(),
            frame_counter: 0,
            time_since_last_update: 0.0,
        }
    }
}

impl ButteryGame for ButteryExample {
    fn get_title(&self) -> String {
        "Butter-Engine Example".into()
    }

    fn on_init(&mut self, state: &mut ButteryEngineState) {
        let components: Vec<Box<dyn ButteryComponent>> =
            vec![Box::new(ExampleComponent::default())];
        let object = Object::new(
            [0.0, 0.0, 0.0],
            [Deg(0.0), Deg(0.0), Deg(0.0)],
            include_bytes!("./models/cube.glb"),
            "./models/cube.glb".into(),
            components,
            &mut state.world_diff,
        );
        state.world_model.objects.insert(object.get_id(), object);

        state.world_model.light = self.light;
    }

    fn on_update(&mut self, state: &mut ButteryEngineState) {
        self.camera_controller
            .update_camera(&mut self.camera, state.delta_time);

        state.world_model.camera = self.camera;

        self.frame_counter += 1;
        self.time_since_last_update += state.delta_time;

        if self.time_since_last_update >= 0.2 {
            self.fps_text = format!(
                "{:.0} FPS",
                self.frame_counter as f32 / self.time_since_last_update
            );

            self.frame_counter = 0;
            self.time_since_last_update = 0.0;
        }

        if self.open_menu {
            state
                .renderer
                .update_ui_model(Some(build_ui_model(self.fps_text.clone())));
        }
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
                state
                    .renderer
                    .update_ui_model(Some(build_ui_model(self.fps_text.clone())));
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

fn build_ui_model(fps: String) -> ButteryUIModel {
    ButteryUIModel {
        windows: vec![ButteryUIWindow {
            max_width: 600.0,
            max_height: 400.0,
            corner_radius: 10.0,
            inner_margin: 16,
            child: ButteryUIElement::Column(vec![ButteryUIElement::Text(fps)]),
            ..Default::default()
        }],
    }
}
