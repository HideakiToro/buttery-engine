use crate::projection::ProjectionType;
use cgmath::{Deg, Point3, Rad};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
    pub fov: f32,
    pub render_distance: f32,
    pub projection: ProjectionType,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
        fov: f32,
        render_distance: f32,
        projection: ProjectionType,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            fov,
            render_distance,
            projection,
        }
    }

    pub fn default() -> Self {
        Self::new(
            (0.0, 0.0, 0.0),
            Deg(0.0),
            Deg(0.0),
            45.0,
            100.0,
            ProjectionType::Perspective,
        )
    }
}
