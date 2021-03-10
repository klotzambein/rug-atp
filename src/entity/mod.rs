// TODO: For every company/resource ID they meet, they add them to their table of
//       met entities and calculate the reward expectation using an RL algorithm.

use std::num::NonZeroU16;

pub mod agent;
pub mod building;
pub mod resources;

use self::{agent::Agent, building::Building, resources::Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(NonZeroU16);

impl EntityId {
    pub fn new(idx: usize) -> EntityId {
        EntityId(NonZeroU16::new((idx + 1) as u16).expect("Agent ID overflow"))
    }

    pub fn as_index(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Entity {
    pub pos: (u16, u16),
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

    pub fn texture(&self) -> i32 {
        match &self.ty {
            EntityType::Agent(a) => a.job.texture(),
            EntityType::Building(Building::Market) => 56,
            EntityType::Building(Building::Hut) => 57,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EntityType {
    Agent(Agent),
    Resource(Resource),
    Building(Building),
}
