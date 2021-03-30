// TODO: For every company/resource ID they meet, they add them to their table of
//       met entities and calculate the reward expectation using an RL algorithm.

use std::num::NonZeroU32;

pub mod agent;
pub mod building;
pub mod resources;

use crate::{config::Config, world::Pos};

use self::{
    agent::{Agent, AgentState},
    building::Building,
    resources::{Resource, ResourceItem},
};

/// This is like a reference to an entity. it contains the index into the entity
/// vector saved in the World struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntityId(NonZeroU32);

impl EntityId {
    /// Create an entity id from an index.
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

/// In our world everything that is not a tile is an entity. Every entity is on
/// exactly one or zero tiles. ANd we have a two way mapping from tile to entity
/// and from entity to tile. An entity can either be an agent, a resource, or a
/// building.
#[derive(Debug, Clone, Hash)]
pub struct Entity {
    /// Position of this entity, should always be on the world or (-1 -1)
    pub pos: Pos,
    /// Type of this entity, this contains all further data.
    pub ty: EntityType,
}

impl Entity {
    /// The texture index of this entity.
    pub fn texture(&self) -> i32 {
        match &self.ty {
            EntityType::Agent(a) => {
                if matches!(a.state, AgentState::DoJob)
                    || matches!(a.job, agent::Job::Fisher { boat: Some(_) })
                {
                    a.job.texture()
                } else {
                    a.home.x.rem_euclid(8) as i32
                }
            }
            EntityType::Building(Building::Market) => 56,
            EntityType::Building(Building::Hut { .. }) => 57,
            EntityType::Building(Building::Boat { .. }) => 49,
            EntityType::Resource(Resource {
                resource: ResourceItem::Wheat,
                ..
            }) => 32,
            EntityType::Resource(Resource {
                resource: ResourceItem::Berry,
                ..
            }) => 33,
            EntityType::Resource(Resource {
                resource: ResourceItem::Meat,
                ..
            }) => 40,
            EntityType::Resource(Resource {
                resource: ResourceItem::Fish,
                ..
            }) => 50,
        }
    }

    /// True if this entity is currently visible.
    pub fn visible(&self) -> bool {
        match self.ty {
            EntityType::Agent(Agent {
                in_building, dead, ..
            }) => !(in_building || dead),
            EntityType::Building(Building::Boat { has_agent }) => !has_agent,
            EntityType::Resource(Resource { timeout, .. }) => timeout == 0,
            _ => true,
        }
    }
}

/// The type of this entity. For more information see Entity.
#[derive(Debug, Clone, Hash)]
pub enum EntityType {
    Agent(Agent),
    Resource(Resource),
    Building(Building),
}

impl EntityType {
    /// This function is called after the entity is generated. And is mainly
    /// used to add agents to buildings.
    pub fn initialize(&mut self, pos: Pos, entities: &mut Vec<Entity>, config: &Config) {
        if let EntityType::Building(b) = self {
            b.initialize(pos, entities, config)
        }
    }
}
