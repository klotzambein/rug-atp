use rand::Rng;

use crate::{entity::EntityId, tile::TileTexture, world::World};

#[derive(Debug, Clone, Default, Hash)]
pub struct Agent {
    pub job_id: u8,
    pub health: u8,
    pub cash: u32,
}

impl Agent {
    pub fn preferred_action(&self, pos: (u16, u16), world: &World) -> AgentAction {
        let dx = rand::thread_rng().gen_range(-1..=1);
        let dy = rand::thread_rng().gen_range(-1..=1);

        let target_x = (pos.0 as i16 + world.width as i16 + dx) as u16 % world.width as u16;
        let target_y = (pos.1 as i16 + world.height as i16 + dy) as u16 % world.height as u16;

        let tt = world.tile_type(target_x, target_y);

        if tt == TileTexture::Grass {
            AgentAction::Move(target_x, target_y)
        } else {
            AgentAction::None
        }
    }
}

pub enum AgentAction {
    None,
    Move(u16, u16),
    Kill(EntityId),
    // Invest(CompanyId),
    Farm(u16, u16),
    Scan(),
}

pub enum JobType {
    None,
    CompanyMember,
    Miner,
    Farmer,
    Explorer,
    Fisher,
    // Builder,
    // Butcher,
}
