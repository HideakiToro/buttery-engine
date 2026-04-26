use cgmath::Deg;
use uuid::Uuid;

use crate::{component::ButteryComponent, registry::Registry};

pub struct ObjectData {
    pub position: [f32; 3],
    pub rotation: [Deg<f32>; 3],
    id: Uuid,
}

impl ObjectData {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}

pub struct Object {
    pub data: ObjectData,
    pub model_buffer: Option<&'static [u8]>,
    pub model_path: String,
    pub components: Vec<Box<dyn ButteryComponent>>,
}

impl Object {
    pub fn default(world_diff: &mut Registry<Object>) -> Self {
        let mut object = Object {
            data: ObjectData {
                position: [0.0, 0.0, 0.0],
                rotation: [Deg(0.0), Deg(0.0), Deg(0.0)],
                id: Uuid::new_v4(),
            },
            model_buffer: None,
            model_path: "".into(),
            components: Vec::new(),
        };

        object.on_init(world_diff);

        object
    }

    pub fn new(
        position: [f32; 3],
        rotation: [Deg<f32>; 3],
        model_buffer: &'static [u8],
        model_path: String,
        components: Vec<Box<dyn ButteryComponent>>,
        world_diff: &mut Registry<Object>,
    ) -> Self {
        let mut object = Object {
            data: ObjectData {
                position,
                rotation,
                id: Uuid::new_v4(),
            },
            model_buffer: Some(model_buffer),
            model_path,
            components,
        };

        object.on_init(world_diff);

        object
    }

    fn on_init(&mut self, world_diff: &mut Registry<Object>) {
        for component in self.components.iter_mut() {
            component.on_init(world_diff, &mut self.data);
        }
    }

    pub fn on_update(&mut self, world_diff: &mut Registry<Object>, delta_time: f32) {
        for component in self.components.iter_mut() {
            component.on_update(world_diff, &mut self.data, delta_time);
        }
    }

    pub fn get_id(&self) -> String {
        self.data.get_id()
    }
}
