use crate::core::{object::Object};

pub trait ButteryRenderer: Send + Sync {
    fn load_model(&mut self, path: &str);

    fn on_update(&mut self, objects: &Vec<Object>, delta_time: f32);

    fn render(&mut self);

    fn resize(&mut self, width: u32, height: u32);
}

pub struct FallbackRenderer {}

impl ButteryRenderer for FallbackRenderer {
    fn load_model(&mut self, _path: &str) { }
    
    fn on_update(&mut self, _objects: &Vec<Object>, _delta_time: f32) { }
    
    fn render(&mut self) { }
    
    fn resize(&mut self, _width: u32, _height: u32) { }
}