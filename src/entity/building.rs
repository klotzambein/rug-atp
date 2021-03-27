use crate::world::Pos;

use super::{agent::Agent, Entity, EntityId, EntityType};

#[derive(Debug, Clone, Hash)]
pub enum Building {
    Market,
    Hut { is_agent_in: bool, agent: EntityId },
    Boat { has_agent: bool },
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
                let mut a = Agent::default();
                a.in_building = true;
                a.state = super::agent::AgentState::BeHome;
                a.home = pos;

                entities.push(Entity {
                    pos,
                    ty: EntityType::Agent(a),
                })
            }
            _ => {}
        }
    }

    pub fn agent_enter(&mut self, id: EntityId) {
        match self {
            Building::Market => {}
            Building::Hut { is_agent_in, agent } => {
                assert_eq!(*agent, id);
                *is_agent_in = true;
            }
            Building::Boat { .. } => {
                panic!("Use EnterBoat action to enter a boat!");
            }
        }
    }

    pub fn agent_leave(&mut self, _id: EntityId) {
        match self {
            Building::Market => {}
            Building::Hut { is_agent_in, .. } => {
                *is_agent_in = false;
            }
            Building::Boat { .. } => {
                panic!("Use LeaveBoat action to leave a boat!");
            }
        }
    }
}
