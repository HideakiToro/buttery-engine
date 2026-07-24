use crate::{
    object::{Object, ObjectData},
    registry::Registry,
};
use std::any::Any;

pub trait ButteryComponent: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn on_init(&mut self, _world_diff: &mut Registry<Object>, _object_data: &mut ObjectData) {}

    fn on_update(
        &mut self,
        _world_diff: &mut Registry<Object>,
        _object_data: &mut ObjectData,
        _delta_time: f32,
    ) {
    }
}
