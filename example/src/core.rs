use crate::component::ExampleComponent;

use super::camera_controller::CameraController;
use buttery_engine::{
    camera::Camera,
    component::ButteryComponent,
    engine::ButteryEngineState,
    game::ButteryGame,
    key_event::{Key, KeyEvent},
    object::Object,
    ui::{
        ButterUI2D, ButteryUIButton, ButteryUIColor, ButteryUIContainer, ButteryUIContainerOutline,
        ButteryUIDirectional, ButteryUIElement, ButteryUIInput, ButteryUIModel, ButteryUIText,
        ButteryUIWindow, ButteryUIWindowRelativePosition,
    },
};
use cgmath::{Deg, Point3, Rad};
use std::f32::consts::PI;

pub struct ButteryExample {
    camera: Camera,
    light: Camera,
    camera_controller: CameraController,
    open_menu: bool,
    fps_text: String,
    secondary_text: String,
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
            secondary_text: "Hello World!".into(),
            frame_counter: 0,
            time_since_last_update: 0.0,
        }
    }

    fn build_hud_model(&mut self) -> ButteryUIModel<ButteryExample> {
        let mut model = ButteryUIModel {
            windows: vec![ButteryUIWindow {
                id: "fps_counter".into(),
                max_width: 100.0,
                max_height: 40.0,
                child: ButteryUIElement::Column(ButteryUIDirectional {
                    children: vec![ButteryUIElement::Text(ButteryUIText {
                        text: self.fps_text.clone(),
                        ..Default::default()
                    })],
                    centered: true,
                    size: None,
                }),
                relative_position: ButteryUIWindowRelativePosition::TopRight,
                offset: ButterUI2D { x: -20.0, y: 20.0 },
                ..Default::default()
            }],
        };

        if !self.open_menu {
            model.windows.push(ButteryUIWindow {
                id: "open_menu_btn".into(),
                max_width: 100.0,
                max_height: 40.0,
                child: ButteryUIElement::Column(ButteryUIDirectional {
                    children: vec![ButteryUIElement::Button(ButteryUIButton {
                        label: "Open Menu".into(),
                        on_click: |game: &mut ButteryExample| {
                            game.open_menu = true;
                        },
                        ..Default::default()
                    })],
                    centered: true,
                    size: None,
                }),
                relative_position: ButteryUIWindowRelativePosition::TopLeft,
                background_color: ButteryUIColor {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                padding: 0,
                offset: ButterUI2D { x: 20.0, y: 20.0 },
                ..Default::default()
            });
        }

        model
    }

    fn build_ui_model(&mut self) -> ButteryUIModel<ButteryExample> {
        let mut model = self.build_hud_model();

        model.windows.push(ButteryUIWindow {
            id: "menu".into(),
            max_width: 250.0,
            max_height: 400.0,
            child: ButteryUIElement::Row(ButteryUIDirectional {
                children: vec![
                    ButteryUIElement::Column(ButteryUIDirectional {
                        children: vec![
                            ButteryUIElement::Button(ButteryUIButton {
                                label: "Test".into(),
                                on_click: |game| {
                                    game.secondary_text = "Test clicked".into();
                                },
                                ..Default::default()
                            }),
                            ButteryUIElement::Container(ButteryUIContainer {
                                children: vec![
                                    ButteryUIElement::Button(ButteryUIButton {
                                        label: "Test".into(),
                                        ..Default::default()
                                    }),
                                    ButteryUIElement::Button(ButteryUIButton {
                                        label: "Test".into(),
                                        ..Default::default()
                                    }),
                                ],
                                color: ButteryUIColor {
                                    r: 255,
                                    g: 0,
                                    b: 0,
                                    a: 255,
                                },
                                corner_radius: 20.0,
                                size: Some(ButterUI2D { x: 10.0, y: 30.0 }),
                                outline: Some(ButteryUIContainerOutline {
                                    color: ButteryUIColor {
                                        a: 255,
                                        r: 255,
                                        g: 255,
                                        b: 255,
                                    },
                                    width: 1.0,
                                }),
                            }),
                            ButteryUIElement::Button(ButteryUIButton {
                                label: "Close Menu".into(),
                                on_click: |game| {
                                    game.open_menu = false;
                                },
                                ..Default::default()
                            }),
                        ],
                        centered: false,
                        size: None,
                    }),
                    ButteryUIElement::Column(ButteryUIDirectional {
                        children: vec![
                            ButteryUIElement::Button(ButteryUIButton {
                                label: "Test".into(),
                                on_click: |game| {
                                    game.secondary_text = "Test clicked".into();
                                },
                                ..Default::default()
                            }),
                            ButteryUIElement::Input(ButteryUIInput {
                                current_value: self.secondary_text.clone(),
                                on_changed: |value, game| {
                                    game.secondary_text = value;
                                },
                                size: Some(ButterUI2D { x: 200.0, y: 50.0 }),
                                background_color: Some(ButteryUIColor {
                                    r: 255,
                                    g: 0,
                                    b: 0,
                                    a: 255,
                                }),
                            }),
                            ButteryUIElement::Input(ButteryUIInput {
                                current_value: self.secondary_text.clone(),
                                on_changed: |value, game| {
                                    game.secondary_text = value;
                                },
                                ..Default::default()
                            }),
                            ButteryUIElement::Input(ButteryUIInput {
                                current_value: self.secondary_text.clone(),
                                on_changed: |value, game| {
                                    game.secondary_text = value;
                                },
                                ..Default::default()
                            }),
                        ],
                        centered: false,
                        size: None,
                    }),
                ],
                centered: true,
                size: None,
            }),
            ..Default::default()
        });

        model
    }
}

impl ButteryGame for ButteryExample {
    fn get_title(&self) -> String {
        "Butter-Engine Example".into()
    }

    fn on_init(&mut self, state: &mut ButteryEngineState<ButteryExample>) {
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

    fn on_update(&mut self, state: &mut ButteryEngineState<ButteryExample>) {
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
            state.renderer.update_ui_model(Some(self.build_ui_model()));
        } else {
            state.renderer.update_ui_model(Some(self.build_hud_model()));
        }
    }

    fn on_key_event(
        &mut self,
        state: &mut ButteryEngineState<ButteryExample>,
        key_event: KeyEvent,
    ) {
        match key_event.key {
            Key::Escape if key_event.pressed => {
                if self.open_menu {
                    self.open_menu = false;
                    state.renderer.update_ui_model(None);
                }
            }
            Key::E if key_event.pressed && !self.open_menu => {
                self.open_menu = true;
                state.renderer.update_ui_model(Some(self.build_ui_model()));
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
