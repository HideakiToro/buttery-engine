use std::any::Any;

use crate::{game::ButteryGame, ui::ButteryUIModel, world_model::ButteryWorldModel};

pub trait ButteryRenderer<G: ButteryGame>: Send + Sync + Any {
    fn load_model(&mut self, buffer: &'static [u8], path: &str);

    fn on_update(&mut self, world_model: &ButteryWorldModel);

    fn render(&mut self, game: &mut G);

    fn resize(&mut self, width: u32, height: u32);

    fn update_ui_model(&mut self, ui_model: Option<ButteryUIModel<G>>);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct FallbackRenderer {}

impl<G: ButteryGame> ButteryRenderer<G> for FallbackRenderer {
    fn load_model(&mut self, _buffer: &'static [u8], _path: &str) {}

    fn on_update(&mut self, _world_model: &ButteryWorldModel) {}

    fn render(&mut self, _game: &mut G) {}

    fn resize(&mut self, _width: u32, _height: u32) {}

    fn update_ui_model(&mut self, _ui_model: Option<ButteryUIModel<G>>) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
