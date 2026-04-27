use crate::{engine::ButteryEngine, game::ButteryGame};

pub trait ButteryWindowingSystem<G: ButteryGame + 'static> {
    fn run(&self, engine: ButteryEngine<G>) -> anyhow::Result<()>;
}
