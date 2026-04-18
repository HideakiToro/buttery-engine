use crate::core::{engine::ButteryEngineState, key_event::KeyEvent};

pub trait ButteryGame {
    fn get_title(&self) -> String;

    fn on_init(&mut self, state: &mut ButteryEngineState);

    fn on_key_event(&mut self, state: &mut ButteryEngineState, key_event: KeyEvent);
}
