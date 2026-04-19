use cgmath::Deg;

use crate::{camera::Camera, object::Object};

pub struct ButteryWorldModel {
    pub camera: Camera,
    pub objects: Vec<Object>,
}

impl ButteryWorldModel {
    pub fn default() -> Self {
        Self {
            camera: Camera::new((0.0, 0.0, 0.0), Deg(0.0), Deg(0.0)),
            objects: vec![],
        }
    }
}
