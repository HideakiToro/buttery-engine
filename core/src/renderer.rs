use std::any::Any;

use crate::{
    game::ButteryGame,
    key_event::MousePosition,
    object::Object,
    ui::{ButteryColor, ButteryUIModel},
    world_model::ButteryWorldModel,
};

pub trait ButteryRenderer<G: ButteryGame>: Any {
    fn set_background_color(&mut self, color: ButteryColor);

    fn load_model(&mut self, path: &str);

    fn unload_model(&mut self, path: &str);

    fn on_update(&mut self, world_model: &ButteryWorldModel);

    fn render(&mut self, game: &mut G);

    fn resize(&mut self, width: u32, height: u32);

    fn update_ui_model(&mut self, ui_model: Option<ButteryUIModel<G>>);

    fn object_at_mouse_position<'a>(
        &self,
        world_model: &'a ButteryWorldModel,
        mouse_position: MousePosition,
    ) -> Option<&'a Object>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct FallbackRenderer {}

impl<G: ButteryGame> ButteryRenderer<G> for FallbackRenderer {
    fn set_background_color(&mut self, _color: ButteryColor) {}

    fn load_model(&mut self, _path: &str) {}

    fn unload_model(&mut self, _path: &str) {}

    fn on_update(&mut self, _world_model: &ButteryWorldModel) {}

    fn render(&mut self, _game: &mut G) {}

    fn resize(&mut self, _width: u32, _height: u32) {}

    fn update_ui_model(&mut self, _ui_model: Option<ButteryUIModel<G>>) {}

    fn object_at_mouse_position<'a>(
        &self,
        _world_model: &'a ButteryWorldModel,
        _mouse_position: MousePosition,
    ) -> Option<&'a Object> {
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
