// TODO: For every company/resource ID they meet, they add them to their table of
//       met entities and calculate the reward expectation using an RL algorithm.

use std::num::NonZeroU32;

pub mod agent;
pub mod building;
pub mod resources;

use crate::world::Pos;

use self::{
    agent::{Agent, AgentState},
    building::Building,
    resources::Resource,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(NonZeroU32);

impl EntityId {
    pub fn new(idx: usize) -> EntityId {
        EntityId(NonZeroU32::new((idx + 1) as u32).expect("Agent ID overflow"))
    }

    pub fn uninitialized() -> EntityId {
        EntityId(NonZeroU32::new(u32::MAX).expect("unreachable"))
    }

    pub fn is_uninitialized(self) -> bool {
        self.0.get() == u32::MAX
    }

    pub fn as_index(self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Entity {
    pub pos: Pos,
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
            EntityType::Agent(a) => {
                if matches!(a.state, AgentState::DoJob) {
                    a.job.texture()
                } else {
                    a.home.x.rem_euclid(8) as i32
                }
            }
            EntityType::Building(Building::Market) => 56,
            EntityType::Building(Building::Hut { .. }) => 57,
            EntityType::Building(Building::Boat { agent: None }) => 49,
            EntityType::Building(Building::Boat { agent: Some(_) }) => 48,
            // TODO IVO: Add the texture indices here. This refers to a texture
            // in assets/characters.png. The indices start at the top left going
            // to the right.
            EntityType::Resource(Resource::Wheat(_)) => 32,
            EntityType::Resource(Resource::Berry(_)) => 33,
            EntityType::Resource(Resource::Meat(_)) => 40,
            EntityType::Resource(Resource::Fish(_)) => 50,
            // _ => unimplemented!(),
        }
    }

    pub fn visible(&self) -> bool {
        match self.ty {
            EntityType::Agent(Agent {
                in_building, dead, ..
            }) => !(in_building || dead),
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum EntityType {
    Agent(Agent),
    Resource(Resource),
    Building(Building),
}

impl EntityType {
    pub fn initialize(&mut self, pos: Pos, entities: &mut Vec<Entity>) {
        match self {
            EntityType::Building(b) => b.initialize(pos, entities),
            _ => {}
        }
    }
}
