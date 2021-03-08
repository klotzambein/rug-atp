// TODO: For every company/resource ID they meet, they add them to their table of
//       met entities and calculate the reward expectation using an RL algorithm.

use std::num::NonZeroU16;

use rand::Rng;

use crate::{agent::Agent, world::World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(NonZeroU16);

impl EntityId {
    pub fn new(idx: usize) -> EntityId {
        EntityId(NonZeroU16::new((idx + 1) as u16).expect("Agent ID overflow"))
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Entity {
    pub pos_x: u16,
    pub pos_y: u16,
    pub ty: EntityType,
}

impl Entity {
    pub fn agent(&self) -> Option<Agent> {
        if let EntityType::Agent(a) = self.ty.clone() {
            Some(a)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EntityType {
    Agent(Agent),
    Resource(),
}
