use buttery_engine::{
    component::ButteryComponent,
    object::{Object, ObjectData},
    registry::Registry,
};
use cgmath::Deg;

#[derive(Default)]
pub struct ExampleComponent {
    angle: f32,
}

impl ButteryComponent for ExampleComponent {
    fn on_init(&mut self, _world_diff: &mut Registry<Object>, object_data: &mut ObjectData) {
        let rand = rand::random::<f32>();
        object_data.position[0] = rand;
    }
    fn on_update(
        &mut self,
        _world_diff: &mut Registry<Object>,
        object_data: &mut ObjectData,
        delta_time: f32,
    ) {
        self.angle += 22.5 * delta_time;
        object_data.rotation[1] = Deg(self.angle);

        object_data.position[0] += 1.0 * delta_time;
        if object_data.position[0] > 2.0 {
            object_data.position[0] -= 4.0;
        }
    }
}
