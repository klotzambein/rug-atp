// TODO: For every company/resource ID they meet, they add them to their table of
//       met entities and calculate the reward expectation using an RL algorithm.

use std::num::NonZeroU16;

use rand::Rng;

use crate::world::World;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AgentId(NonZeroU16);

impl AgentId {
    pub fn new(idx: usize) -> AgentId {
        AgentId(NonZeroU16::new((idx + 1) as u16).expect("Agent ID overflow"))
    }
}

#[derive(Debug, Clone, Default, Hash)]
pub struct Agent {
    pub pos_x: u16,
    pub pos_y: u16,
    pub job_id: u8,
    pub health: u8,
    pub cash: u32,
}

impl Agent {
    pub fn preferred_action(&self, world: &World) -> AgentAction {
        let tt = world.tile_type(self.pos_x, self.pos_y);
        
        let target_x = rand::thread_rng().gen_range(0..world.width);
        let target_y = rand::thread_rng().gen_range(0..world.height);
        if self.pos_x == 0 || self.pos_y == 0 {
            AgentAction::Move(target_x as u16, target_y as u16)
        // else if {
        } else {
            AgentAction::None
        }
    }
}

pub enum AgentAction {
    None,
    Move(u16, u16),
    Kill(AgentId),
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