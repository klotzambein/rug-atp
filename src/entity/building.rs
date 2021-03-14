use crate::world::Pos;

use super::{agent::Agent, Entity, EntityId, EntityType};

#[derive(Debug, Clone, Hash)]
pub enum Building {
    Market,
    Hut { is_agent_in: bool, agent: EntityId },
}

impl Building {
    pub fn hut_uninitialized() -> Building {
        Building::Hut {
            is_agent_in: true,
            agent: EntityId::uninitialized(),
        }
    }

    pub fn initialize(&mut self, pos: Pos, entities: &mut Vec<Entity>) {
        match self {
            Building::Hut {
                is_agent_in: _,
                agent,
            } if agent.is_uninitialized() => {
                *agent = EntityId::new(entities.len());
                entities.push(Entity {
                    pos,
                    in_building: true,
                    ty: EntityType::Agent(Agent::default()),
                })
            }
            _ => {}
        }
    }

    pub fn agent_leave(&mut self, _id: EntityId) {
        match self {
            Building::Market => {}
            Building::Hut { is_agent_in, .. } => {
                *is_agent_in = false;
            }
        }
    }
}
