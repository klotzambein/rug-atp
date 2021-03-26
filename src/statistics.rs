use crate::{entity::resources::PerResource, world::World};
#[derive(Debug, Default, Clone)]
pub struct Statistics {
    pub prices: PerResource<Vec<f32>>,
    pub agent_count: Vec<f32>,
}

impl Statistics {
    pub fn step(&mut self, world: &World) {
        let prices = world
            .market
            .prices()
            .map(|p| p.map(|p| p as f32).unwrap_or(f32::NAN));
        for (r, p) in self.prices.iter_mut() {
            p.push(prices[r]);
        }
        self.agent_count.push(world.alive_count as f32);
    }
}