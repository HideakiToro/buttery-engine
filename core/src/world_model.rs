use std::collections::HashMap;

use cgmath::Deg;

use crate::{camera::Camera, object::Object, registry::Registry};

pub struct ButteryWorldModel {
    pub camera: Camera,
    pub objects: HashMap<String, Object>,
}

impl ButteryWorldModel {
    pub fn default() -> Self {
        Self {
            camera: Camera::new((0.0, 0.0, 0.0), Deg(0.0), Deg(0.0)),
            objects: HashMap::new(),
        }
    }

    pub fn apply_diff(&mut self, world_diff: &mut Registry<Object>) {
        self.objects
            .retain(|key, _| !world_diff.to_delete.contains_key(key));

        for (key, obj) in world_diff.to_create.drain() {
            self.objects.insert(key, obj);
        }
    }
}
