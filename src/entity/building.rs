use crate::{config::Config, world::Pos};

use super::{agent::Agent, Entity, EntityId, EntityType};

/// There are three types of buildings in the world:
/// - Markets: Here agents go to trade.
/// - Hut: every agent has exactly one hut they call home.
/// - Boat: These are used by fishers to go fishing.
#[derive(Debug, Clone, Hash)]
pub enum Building {
    Market,
    Hut { is_agent_in: bool, agent: EntityId },
    Boat { has_agent: bool },
}

impl Building {
    /// Create an uninitialized hut (without an agent). Huts should be
    /// initialized, by calling initialize later.
    pub fn hut_uninitialized() -> Building {
        Building::Hut {
            is_agent_in: true,
            agent: EntityId::uninitialized(),
        }
    }

    /// Initialize the building (add agent to hut).
    pub fn initialize(&mut self, pos: Pos, entities: &mut Vec<Entity>, config: &Config) {
        match self {
            Building::Hut {
                is_agent_in: _,
                agent,
            } if agent.is_uninitialized() => {
                *agent = EntityId::new(entities.len());
                let mut a = Agent::new(config);
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

    /// This is called when an agent enters a building.
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

    /// This is called when an agent leaves a building.
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
