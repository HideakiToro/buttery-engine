use crate::{
    engine::ButteryEngineState,
    key_event::{KeyEvent, MousePosition},
};

pub trait ButteryGame: Sized + 'static {
    fn get_title(&self) -> String;

    fn on_init(&mut self, state: &mut ButteryEngineState<Self>);

    fn on_update(&mut self, state: &mut ButteryEngineState<Self>);

    fn on_key_event(&mut self, state: &mut ButteryEngineState<Self>, key_event: KeyEvent);

    #[allow(unused_variables)]
    fn on_mouse_moved(
        &mut self,
        state: &mut ButteryEngineState<Self>,
        mouse_position: MousePosition,
    ) {
    }
}
