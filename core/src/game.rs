use crate::{engine::ButteryEngineState, key_event::KeyEvent};

pub trait ButteryGame: Sized + 'static {
    fn get_title(&self) -> String;

    fn on_init(&mut self, state: &mut ButteryEngineState<Self>);

    fn on_update(&mut self, state: &mut ButteryEngineState<Self>);

    fn on_key_event(&mut self, state: &mut ButteryEngineState<Self>, key_event: KeyEvent);
}
