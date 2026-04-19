use crate::engine::ButteryEngine;

pub trait ButteryWindowingSystem {
    fn run(&self, engine: ButteryEngine) -> anyhow::Result<()>;
}
