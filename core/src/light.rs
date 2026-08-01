use cgmath::{Deg, Point3, Rad};

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Point3<f32>,
    pub l_type: LightType,
    pub render_distance: f32,
}

impl Light {
    pub fn new<V: Into<Point3<f32>>>(position: V, render_distance: f32, l_type: LightType) -> Self {
        Self {
            position: position.into(),
            render_distance,
            l_type,
        }
    }

    pub fn default() -> Self {
        Self::new(
            (0.0, 0.0, 0.0),
            100.0,
            LightType::Directional(LightDirection::new(Deg(-180.0), Deg(-35.0))),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Directional(LightDirection),
}

#[derive(Debug, Clone, Copy)]
pub struct LightDirection {
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}

impl LightDirection {
    pub fn new<R: Into<Rad<f32>>>(yaw: R, pitch: R) -> Self {
        Self {
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
}
