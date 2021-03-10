use rand::Rng;

use crate::{entity::EntityId, tile::TileTexture, world::World};

#[derive(Debug, Clone, Hash)]
pub struct Agent {
    pub job: Job,
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

// TODO: Move to company.
type CompanyId = u8;

#[derive(Debug, Clone, Hash)]
pub enum Job {
    None,
    CompanyMember(CompanyId),
    Miner,
    Farmer,
    Explorer,
    Fisher,
    // Builder,
    // Butcher,
}

impl Job {
    pub fn texture(&self) -> i32 {
        match self {
            Job::None => 0,
            Job::CompanyMember(c) => *c as i32 + 8,
            Job::Miner => 2,
            Job::Farmer => 3,
            Job::Explorer => 4,
            Job::Fisher => 5,
        }
    }
}
