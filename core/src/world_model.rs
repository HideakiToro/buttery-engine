use std::collections::HashMap;

use crate::{camera::Camera, light::Light, object::Object, registry::Registry};

pub struct ButteryWorldModel {
    pub camera: Camera,
    pub light: Light,
    pub objects: HashMap<String, Object>,
}

impl ButteryWorldModel {
    pub fn default() -> Self {
        Self {
            camera: Camera::default(),
            light: Light::default(),
            objects: HashMap::new(),
        }
    }

    pub fn apply_diff(&mut self, world_diff: &mut Registry<Object>) {
        if !world_diff.to_delete.is_empty() {
            self.objects
                .retain(|key, _| !world_diff.to_delete.contains(key));
        }

        for (key, obj) in world_diff.to_create.drain() {
            self.objects.insert(key, obj);
        }

        world_diff.reset();
    }
}
