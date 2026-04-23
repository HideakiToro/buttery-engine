use crate::{
    object::{Object, ObjectData},
    registry::Registry,
};

pub trait ButteryComponent {
    fn on_init(&mut self, _world_diff: &mut Registry<Object>, _object_data: &mut ObjectData) {}
    fn on_update(
        &mut self,
        _world_diff: &mut Registry<Object>,
        _object_data: &mut ObjectData,
        _delta_time: f32,
    ) {
    }
}
