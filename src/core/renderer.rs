use crate::core::{ui::ButteryUIModel, world_model::ButteryWorldModel};

pub trait ButteryRenderer: Send + Sync {
    fn load_model(&mut self, path: &str);

    fn on_update(&mut self, world_model: &ButteryWorldModel);

    fn render(&mut self);

    fn resize(&mut self, width: u32, height: u32);

    fn update_ui_model(&mut self, ui_model: Option<ButteryUIModel>);
}

pub struct FallbackRenderer {}

impl ButteryRenderer for FallbackRenderer {
    fn load_model(&mut self, _path: &str) {}

    fn on_update(&mut self, _world_model: &ButteryWorldModel) {}

    fn render(&mut self) {}

    fn resize(&mut self, _width: u32, _height: u32) {}

    fn update_ui_model(&mut self, _ui_model: Option<ButteryUIModel>) {}
}
