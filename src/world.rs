//! This module contains the World struct which holds all the simulation state.
//! This file ties together the simulation of agents, markets, etc. and performs
//! some other tasks.

use std::rc::Rc;

use dear_gui::graphics::primitives::{Sprite, Vf2};
use glium::Display;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    config::Config,
    entity::{agent::Job, Entity, EntityId, EntityType},
    entity::{
        agent::{Agent, AgentAction},
        building::Building,
        resources::Resource,
    },
    generation::BiomeMap,
    grid::CanvasGrid,
    market::Market,
    statistics::Statistics,
    tile::TileType,
};

/// This struct holds all the simulation state. And defines the root step
/// function.
pub struct World {
    /// Configuration of this simulation
    pub config: Rc<Config>,
    /// The tile types of the entire map. The length is width x height. For more
    /// details see tile.rs
    pub tiles_type: Vec<TileType>,
    /// The tile entities of the entire map. The length is width x height. For
    /// more details see tile.rs
    pub tiles_entity: Vec<Option<EntityId>>,
    /// All the entities are in this array. This is in one place to hopefully
    /// improve cache locality. Entities do not get added or deleted after
    /// initialization.
    entities: Vec<Entity>,
    /// The global market
    pub market: Market,
    /// Width of the world in tiles
    pub width: usize,
    /// Height of the world in tiles
    pub height: usize,
    /// This flag will be set if the tiles_type variable changes, so we can
    /// avoid having to reupload the chunks to the gpu.
    pub dirty: bool,
    /// Current tick counter.
    pub tick: u32,
    /// Is the simulation running?
    pub is_running: bool,
    /// Running count of alive agents. Not correct while stepping.
    pub alive_count: u32,
    /// Count of alive agents at the beginning of the simulation.
    pub start_count: u32,
}

impl World {
    pub fn new(
        width: usize,
        height: usize,
        rng: &mut impl Rng,
        config: Rc<Config>,
        stats: &mut Statistics,
    ) -> World {
        // Generate the terrain using Noise functions and tile distributions
        let biomes = BiomeMap::new(&config);

        let mut entities = Vec::new();
        let mut tiles_entity = vec![None; width * height];
        let tiles_type = (0..width * height)
            .map(|i| {
                let pos = Pos::new((i % width) as i16, (i / width) as i16);
                let (tt, e) = biomes.get(pos, rng);
                if let Some(mut e) = e {
                    e.initialize(pos, &mut entities, &config);
                    entities.push(Entity { pos, ty: e });
                    tiles_entity[i] = Some(EntityId::new(entities.len() - 1))
                }
                tt
            })
            .take(width * height)
            .collect::<Vec<_>>();

        let start_count = entities
            .iter()
            .filter(|e| matches!(e.ty, EntityType::Agent(_)))
            .count() as u32;

        // Initialize the stats agents
        stats.init_agents(&entities);

        World {
            tiles_type,
            tiles_entity,
            entities,
            market: Market::default(),
            width,
            height,
            dirty: true,
            tick: 0,
            is_running: true,
            alive_count: 0,
            start_count,
            config,
        }
    }

    /// Get an index into the tiles vectors from a position.
    pub fn idx(&self, p: Pos) -> usize {
        let x = p.x as usize;
        let y = p.y as usize;
        debug_assert!(x < self.width && y < self.height);
        x + y * self.width
    }

    /// Get all the neighboring positions. These positions are properly wrapped.
    pub fn neighbors(&self, pos: Pos) -> [Pos; 8] {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let left = ((x + self.width - 1) % self.width) as i16;
        let right = ((x + 1) % self.width) as i16;
        let bot = ((y + self.width - 1) % self.width) as i16;
        let top = ((y + 1) % self.width) as i16;
        let x = x as i16;
        let y = y as i16;
        [
            Pos::new(left, top),
            Pos::new(x, top),
            Pos::new(right, top),
            Pos::new(left, y),
            Pos::new(right, y),
            Pos::new(left, bot),
            Pos::new(x, bot),
            Pos::new(right, bot),
        ]
    }

    /// Wrap a position to be on the world.
    pub fn wrap_pos(&self, x: isize, y: isize) -> Pos {
        Pos::new(
            x.rem_euclid(self.width as isize) as i16,
            y.rem_euclid(self.width as isize) as i16,
        )
    }

    /// This function searches for a tile around the given position, by going
    /// around in a spiral. We will call the closure until it returns true, or we
    /// reach n tiles.
    pub fn find_tile_around(
        &self,
        p: Pos,
        n: usize,
        mut f: impl FnMut(Pos) -> bool,
    ) -> Option<Pos> {
        (2..)
            .map(|i| (i / 2, i % 4))
            .flat_map(|(n, d)| std::iter::repeat(d).take(n))
            .scan((p.x as isize, p.y as isize), |p, d| {
                let pos = self.wrap_pos(p.0, p.1);
                match d {
                    0 => *p = (p.0, p.1 + 1),
                    1 => *p = (p.0 + 1, p.1),
                    2 => *p = (p.0, p.1 - 1),
                    3 => *p = (p.0 - 1, p.1),
                    _ => unreachable!(),
                }
                Some(pos)
            })
            .take(n)
            .find(|p| f(*p))
    }

    /// Wrapper around [`World::find_tile_around`] to find an entity.
    pub fn find_entity_around(
        &self,
        p: Pos,
        n: usize,
        mut f: impl FnMut(&Entity) -> bool,
    ) -> Option<Pos> {
        self.find_tile_around(p, n, |p| {
            if let Some(e) = self.entity_at(p) {
                f(e)
            } else {
                false
            }
        })
    }

    /// Get a reference to an entity from an entity
    pub fn entity(&self, id: EntityId) -> &Entity {
        &self.entities[id.as_index()]
    }

    /// Execute one step, if the simulation is running.
    pub fn step(&mut self, stats: &mut Statistics) {
        if self.is_running {
            self.step_once(stats)
        }
    }

    /// Select a random agent that is alive.
    pub fn random_alive(&self) -> Option<EntityId> {
        self.entities
            .iter()
            .enumerate()
            .filter(|(_, e)| matches!(e.ty, EntityType::Agent(Agent { dead: false, .. })))
            .map(|(i, _)| EntityId::new(i))
            .choose(&mut thread_rng())
    }

    /// Step one step, regardless of the running flag.
    pub fn step_once(&mut self, stats: &mut Statistics) {
        // Step the market, this resets the demand and recalculates the order prices.
        {
            let es = &mut self.entities;
            self.market.step(self.tick, |o, r| {
                if let EntityType::Agent(a) = &mut es[o.agent.as_index()].ty {
                    a.collect(r, o.amount);
                }
            });
        }

        // set the alive count to zero, we add one for every agent
        self.alive_count = 0;

        // Step all the entities
        for i in 0..self.entities.len() {
            let mut entity = self.entities[i].clone();
            let id = EntityId::new(i);
            match &mut entity.ty {
                EntityType::Agent(a) => {
                    self.step_agent(a, &mut entity.pos, id);
                    // record statistics for this agent
                    stats.step_agent(&a, id.as_index());
                }
                EntityType::Resource(r) => {
                    self.step_resource(r, &mut entity.pos, i);
                }
                EntityType::Building(_) => (),
            }
            self.entities[i] = entity;
        }

        // increase tick counter.
        self.tick += 1;

        // step the statistics recording most values
        stats.step(self);
    }

    /// This function ties the agents into the simulation, also look at Agent::step.
    fn step_agent(&mut self, a: &mut Agent, pos: &mut Pos, id: EntityId) {
        // do nothing if the agent is dead
        if a.dead {
            return;
        }

        self.alive_count += 1;

        let current_tile_idx = self.idx(*pos);

        // Execute the agent action
        match a.step(*pos, &self) {
            AgentAction::Move(p) => {
                assert!(!a.in_building);
                assert!(a.can_walk_on(p, self), "{:#?}", a);
                let idx = self.idx(p);
                self.tiles_entity[current_tile_idx] = None;
                self.tiles_entity[idx] = Some(id);
                *pos = p;
            }
            AgentAction::Leave(p) => {
                assert!(a.in_building);
                assert!(a.can_walk_on(p, self), "{:#?}", a);

                // Modify agent entity
                a.in_building = false;
                *pos = p;

                // Set destination tile entity
                let idx = self.idx(p);
                self.tiles_entity[idx] = Some(id);

                // Modify building
                let building_entity_id = self.tiles_entity[current_tile_idx].unwrap();
                let building_entity = &mut self.entities[building_entity_id.as_index()];
                if let EntityType::Building(b) = &mut building_entity.ty {
                    b.agent_leave(id);
                } else {
                    panic!("Not a building");
                }
            }
            AgentAction::Enter(p) => {
                assert!(!a.in_building);

                // Clear source tile entity
                self.tiles_entity[current_tile_idx] = None;

                // Modify agent entity
                a.in_building = true;
                *pos = p;

                // Modify building
                let idx = self.idx(p);
                let building_entity_id = self.tiles_entity[idx].unwrap();
                let building_entity = &mut self.entities[building_entity_id.as_index()];
                if let EntityType::Building(b) = &mut building_entity.ty {
                    b.agent_enter(id);
                } else {
                    panic!("Not a building");
                }
            }
            AgentAction::EnterBoat(p) => {
                assert!(!a.in_building);

                // Clear source tile entity
                self.tiles_entity[current_tile_idx] = None;

                // Modify building
                let idx = self.idx(p);
                let boat_entity_id = self.tiles_entity[idx].unwrap();
                let boat_entity = &mut self.entities[boat_entity_id.as_index()];
                if let EntityType::Building(Building::Boat { has_agent }) = &mut boat_entity.ty {
                    *has_agent = true;
                } else {
                    panic!("Not a boat");
                }
                self.tiles_entity[idx] = Some(id);

                // Modify agent entity
                *pos = p;
                if let Job::Fisher { boat } = &mut a.job {
                    assert!(boat.is_none());
                    *boat = Some(boat_entity_id);
                } else {
                    panic!("Not a fisher")
                }
            }
            AgentAction::LeaveBoat(p) => {
                assert!(!a.in_building);

                if let Job::Fisher { boat } = &mut a.job {
                    let b_id = boat.unwrap();

                    // Modify building
                    let boat_entity = &mut self.entities[b_id.as_index()];
                    if let EntityType::Building(Building::Boat { has_agent }) = &mut boat_entity.ty
                    {
                        *has_agent = false;
                    } else {
                        panic!("Not a boat");
                    }
                    boat_entity.pos = *pos;
                    self.tiles_entity[current_tile_idx] = Some(b_id);

                    // Modify agent entity
                    *pos = p;
                    *boat = None;
                } else {
                    panic!("Not a fisher")
                }
            }
            AgentAction::Farm(p) => {
                // Modify resource
                let idx = self.idx(p);
                let resource_entity_id = self.tiles_entity[idx].unwrap();
                let resource_entity = &mut self.entities[resource_entity_id.as_index()];
                let resource_farmed = if let EntityType::Resource(r) = &mut resource_entity.ty {
                    r.farm()
                } else {
                    panic!("Not a resource {:?}", resource_entity);
                };

                // Modify agent entity
                if let Some(resource) = resource_farmed {
                    a.collect(resource, 1)
                }
            }
            AgentAction::Consume(r, q) => a.consume(r, q, &self.config),
            AgentAction::MarketOrder {
                item,
                price,
                amount,
            } => {
                // Tke the resources and create an order on the market
                let inventory = &mut a.inventory[item];
                *inventory = inventory.checked_sub(amount).unwrap();
                self.market.order(id, item, price, amount);
            }
            AgentAction::MarketPurchase { item, amount } => {
                let (agents, resources_gained) = self.market.buy(item, amount, a.cash);
                // Collect the new resources
                a.collect(item, resources_gained);

                // Transfer money to all agents we bought resources from
                for (agent, price) in agents {
                    a.cash = a.cash.checked_sub(price).unwrap();

                    if let Entity {
                        ty: EntityType::Agent(b),
                        ..
                    } = &mut self.entities[agent.as_index()]
                    {
                        b.cash += price as u32;
                    } else {
                        panic!()
                    }
                }
            }
            // Do nothing this step
            AgentAction::None => {}
            AgentAction::Die => {
                if a.in_building {
                    // Leave the building before the agent dies
                    let building_entity_id = self.tiles_entity[current_tile_idx].unwrap();
                    let building_entity = &mut self.entities[building_entity_id.as_index()];
                    if let EntityType::Building(b) = &mut building_entity.ty {
                        b.agent_leave(id);
                    } else {
                        panic!("Not a building");
                    }
                } else {
                    self.tiles_entity[current_tile_idx] = None;
                }
                a.dead = true;
                *pos = Pos::new(-1, -1);
            }
        }
    }

    // This is called for every resource every tick
    fn step_resource(&mut self, r: &mut Resource, pos: &mut Pos, idx: usize) {
        let current_tile_idx = self.idx(*pos);

        // Resources will be removed from the map once they are empty, they will
        // then be respawned after timeout ticks.
        if r.available() == 0 {
            if r.timeout == 0 {
                r.timeout = self.config.resource_timeout;
                self.tiles_entity[current_tile_idx] = None;
            } else if r.timeout == 1 {
                // Check that we are not respawning on an agent.
                if self.tiles_entity[current_tile_idx].is_none() {
                    self.tiles_entity[current_tile_idx] = Some(EntityId::new(idx));
                    r.timeout = 0;
                    r.amount = (thread_rng().sample::<f32, _>(rand_distr::StandardNormal)
                        * self.config.resource_amount_sd
                        + self.config.resource_amount_mean)
                        .max(0.) as u16;
                }
            } else {
                r.timeout -= 1;
            }
        }
    }

    /// This function is used by the interactive mode to update the vertex
    /// buffers and redraw the sprites.
    pub fn update_grid(&mut self, display: &Display, grid: &mut CanvasGrid) {
        assert_eq!(grid.width * 32, self.width);
        assert_eq!(grid.height * 32, self.height);
        if self.dirty {
            for cx in 0..grid.width {
                for cy in 0..grid.height {
                    let start = cy * 32 * self.width + cx * 32;
                    grid.update_chunk(
                        (cx, cy),
                        (0..32).flat_map(|y| {
                            self.tiles_type[start + y * self.width..start + y * self.width + 32]
                                .iter()
                                .copied()
                        }),
                    )
                }
            }
            self.dirty = false;
        }

        grid.update_agents(
            display,
            self.entities
                .iter()
                .filter(|e| e.visible())
                .map(|a| Sprite {
                    vertex: Vf2::new(a.pos.x as f32 * 10., a.pos.y as f32 * 10.),
                    size: Vf2::new(10., 10.),
                    texture_index: a.texture(),
                }),
        )
    }

    /// Get the tile type at a given position
    pub fn tile_type(&self, p: Pos) -> TileType {
        self.tiles_type[self.idx(p)]
    }

    /// Get the entity at a given position
    pub fn entity_at(&self, pos: Pos) -> Option<&Entity> {
        let e = self.tiles_entity[self.idx(pos)]?;
        Some(&self.entities[e.as_index()])
    }

    /// Check weather an agent can walk on the given tile.
    pub fn tile_is_walkable(&self, p: Pos) -> bool {
        self.tile_type(p).walkable() && self.entity_at(p).is_none()
    }

    /// Check weather a boat can sail on the given tile. Boats can also move on
    /// sand, to allow being parked on the beach.
    pub fn tile_is_sailable(&self, p: Pos) -> bool {
        let typ = self.tile_type(p);
        (typ == TileType::Water || typ == TileType::Sand) && self.entity_at(p).is_none()
    }

    /// The days start at 0 and generally last 200 ticks.
    pub fn time_of_day(&self) -> u32 {
        self.tick % self.config.day_length
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    pub fn new(x: i16, y: i16) -> Pos {
        Pos { x, y }
    }

    pub fn wrap(self, world: &World) -> Self {
        Pos::new(
            self.x.rem_euclid(world.width as i16),
            self.y.rem_euclid(world.height as i16),
        )
    }

    pub fn is_adjacent(self, other: Pos) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}
