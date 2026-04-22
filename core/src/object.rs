use crate::component::ButteryComponent;

pub struct Object {
    pub position: [f32; 3],
    pub model_path: String,
    pub components: Vec<Box<dyn ButteryComponent>>,
}

impl Object {
    pub fn default() -> Self {
        let mut object = Object {
            position: [0.0, 0.0, 0.0],
            model_path: "".into(),
            components: Vec::new(),
        };

        object.on_init();

        object
    }

    pub fn new(
        position: [f32; 3],
        model_path: String,
        components: Vec<Box<dyn ButteryComponent>>,
    ) -> Self {
        let mut object = Object {
            position,
            model_path,
            components,
        };

        object.on_init();

        object
    }

    fn on_init(&mut self) {
        for component in self.components.iter_mut() {
            component.on_init();
        }
    }

    pub fn on_update(&mut self) {
        for component in self.components.iter_mut() {
            component.on_update();
        }
    }
}
