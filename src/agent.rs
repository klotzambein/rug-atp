use std::num::NonZeroU16;

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
    pub health: u8,
    pub cash: u32,
}

impl Agent {
    pub fn preferred_action(&self, world: &World) -> AgentAction {
        let tt = world.tile_type(self.pos_x, self.pos_y);
        if self.pos_x != 10 || self.pos_y != 10 {
            AgentAction::Move(10, 10)
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
