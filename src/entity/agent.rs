use std::cmp::Ordering;

use rand::{
    distributions::{Bernoulli, Standard},
    prelude::*,
    Rng,
};

use crate::world::{Pos, World};

use super::{
    building::Building,
    resources::{PerResource, ResourceItem},
    EntityType,
};

#[derive(Debug, Default, Clone, Hash)]
pub struct Agent {
    /// This contains the agents job, and all variables associated with said
    /// job.
    pub job: Job,
    /// This contains the nutritional value of each food resource. This value
    /// decreases when the food is eaten and increases when a different food is
    /// eaten.
    pub nutrition: PerResource<u8>,
    /// This contains the amount of resources the agent possesses at the moment.
    pub inventory: PerResource<u16>,
    /// This is the agents current energy. This value is between 0 and 10_000
    pub energy: u16,
    /// This is the agents current cash. This can be used to buy resources at
    /// the market.
    pub cash: u32,
    /// This is true when the agent is in a building. To check which building
    /// the agent is in look up the current position in the world.
    pub in_building: bool,
    /// If this is true the agent is dead.
    pub dead: bool,
}

impl Agent {
    pub fn step(&mut self, pos: Pos, world: &World) -> AgentAction {
        if self.dead {
            return AgentAction::None;
        }

        self.energy = self.energy.saturating_sub(1);
        if self.energy == 0 {
            return AgentAction::Die;
        }

        if self.in_building {
            if let Some(p) = world.find_tile_around(pos, 9, |p| world.tile_is_walkable(p)) {
                return AgentAction::Leave(p);
            } else {
                return AgentAction::None;
            }
        }

        self.do_job(pos, world)
    }

    pub fn do_job(&mut self, pos: Pos, world: &World) -> AgentAction {
        match &mut self.job {
            Job::None => AgentAction::None,
            Job::Lumberer => self.find_and_farm(world, pos, ResourceItem::Berry),
            Job::Farmer => self.find_and_farm(world, pos, ResourceItem::Wheat),
            Job::Butcher => self.find_and_farm(world, pos, ResourceItem::Meat),
            // Job::FisherBoat => self.find_and_farm(world, pos, ResourceItem::Fish),
            Job::Fisher => {
                // First find a boat and enter it
                // if !self.in_boat {
                //     let next_action = self.find_and_farm(
                //         world,
                //         pos,
                //         EntityType::Building(Building::Boat { agent: None }),
                //     );
                //     match next_action {
                //         AgentAction::Enter(_) => {
                //             self.job = Job::FisherBoat;
                //             self.in_boat = true;
                //             // find closest water tile
                //             // move to water: idk how??
                //             // only move on water now

                //             return next_action;
                //         }
                //     }
                // }
                AgentAction::None
            }
            Job::Explorer {
                observations,
                count,
            } => {
                world.find_entity_around(pos, 15 * 15, |e| {
                    // matches!(e.ty, EntityType::Resource(Resource::Berry(_)))
                    match &e.ty {
                        EntityType::Resource(r) => {
                            observations[r.product()] += r.available() as u16 / 10
                        }
                        EntityType::Building(Building::Boat { .. }) => observations.fish += 30,
                        _ => (),
                    }
                    false
                });

                *count += 1;
                if *count == 200 {
                    let mut max_freq: u16 = 0;
                    let mut best_item: ResourceItem = ResourceItem::Berry;
                    for (resource, observation) in observations.iter() {
                        if *observation > max_freq {
                            max_freq = *observation;
                            best_item = resource;
                        }
                    }

                    match best_item {
                        ResourceItem::Berry => self.job = Job::Lumberer,
                        ResourceItem::Wheat => self.job = Job::Farmer,
                        ResourceItem::Fish => self.job = Job::Fisher,
                        ResourceItem::Meat => self.job = Job::Butcher,
                    }
                }

                let dir: Direction = rand::random();

                let target = (pos + dir).wrap(world);

                if world.tile_is_walkable(target) {
                    AgentAction::Move(target)
                } else {
                    AgentAction::None
                }
            }
        }
    }

    /// This function will return actions that lead to the agents locating a resource and farming it.
    pub fn find_and_farm(&mut self, world: &World, pos: Pos, item: ResourceItem) -> AgentAction {
        let search_radius = 15;

        let target_pos = world.find_entity_around(pos, search_radius * search_radius, |e| {
            if let EntityType::Resource(r) = &e.ty {
                r.produces_item(item)
            } else {
                false
            }
        });

        let pf = self.path_find(pos, target_pos, world);

        match pf {
            Ok(target) => AgentAction::Farm(target),
            Err(Some(pos)) => AgentAction::Move(pos),
            Err(None) => AgentAction::None,
        }
    }

    /// This function makes the agent path find towards a given position. Should
    /// no position be given the agent will walk around randomly.
    ///
    /// # Return value
    /// - Ok(pos) => The agent is right next to the target pos
    /// - Err(Some(pos)) => Move to the given pos next
    /// - Err(None) => The agent can not move
    pub fn path_find(
        &mut self,
        pos: Pos,
        target: Option<Pos>,
        world: &World,
    ) -> Result<Pos, Option<Pos>> {
        let mut rng = rand::thread_rng();
        let unstuckifier = Bernoulli::new(0.75).unwrap();

        if let Some(target) = target {
            if target.is_adjacent(pos) {
                return Ok(target);
            }
            if unstuckifier.sample(&mut rng) {
                let move_dir = Direction::delta(pos, target);
                let next_pos = pos + move_dir;
                if world.tile_is_walkable(next_pos) {
                    return Err(Some(next_pos));
                }
            }
        }

        let next = world
            .neighbors(pos)
            .iter()
            .cloned()
            .filter(|p| world.tile_is_walkable(*p))
            .choose(&mut rng);

        match next {
            Some(n) => Err(Some(n)),
            None => Err(None),
        }
    }

    /// This function will add the given resource to the agents inventory.
    pub fn collect(&mut self, resource: ResourceItem) {
        self.inventory[resource] += 1;
    }

    pub fn consume(&mut self, resource: ResourceItem) {
        assert!(self.inventory[resource] > 0);
        self.inventory[resource] -= 1;
        self.energy += self.nutrition[resource] as u16;
        if self.energy > 10000 {
            self.energy = 10000;
        }
        for (r, n) in self.nutrition.iter_mut() {
            if r == resource {
                *n = n.saturating_sub(9);
            } else {
                *n = n.saturating_add(4);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum AgentAction {
    /// Do nothing this step
    None,
    /// Try to move to the given position
    Move(Pos),
    /// Mine a resource at the given position
    Farm(Pos),
    /// Enter a building at pos
    Enter(Pos),
    /// Leave a building and go to pos
    Leave(Pos),
    /// This is only valid if an agent is in a market. This action will create
    /// an order at the given price with the specified amount
    MarketOrder {
        item: ResourceItem,
        price: u16,
        amount: u16,
    },
    /// This is only valid if an agent is in a market. This action will purchase
    /// the given item at the cheapest market price.
    MarketPurchase { item: ResourceItem, amount: u16 },
    /// Die: remove this agent from this agent from the world and set its dead
    /// flag to true.
    Die,
}

#[derive(Debug, Clone, Hash)]
pub enum Job {
    None,
    Explorer {
        count: u8,
        observations: PerResource<u16>,
    },
    Farmer,
    Lumberer,
    Fisher,
    // FisherBoat,
    Butcher,
    // Miner,
    // CompanyMember(CompanyId),
    // Builder,
}

impl Job {
    pub fn texture(&self) -> i32 {
        match self {
            Job::None => 0,
            // Job::CompanyMember(c) => *c as i32 + 8,
            // Job::Miner => 2,
            Job::Farmer => 10,
            Job::Explorer { .. } => 11,
            Job::Fisher => 12,
            // Job::FisherBoat => 51,
            Job::Butcher => 13,
            Job::Lumberer => 15,
        }
    }
}

impl Default for Job {
    fn default() -> Self {
        Job::Lumberer
    }
}

impl Distribution<Job> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Job {
        match rng.gen_range(0..4) {
            0 => Job::Explorer {
                observations: Default::default(),
                count: 0,
            },
            1 => Job::Farmer,
            2 => Job::Butcher,
            3 => Job::Lumberer,
            // 4 => Job::Fisher,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn delta(end: Pos, start: Pos) -> Direction {
        let dx = start.x.cmp(&end.x);
        let dy = start.y.cmp(&end.y);

        match (dx, dy) {
            (Ordering::Less, Ordering::Less) => Direction::DownLeft,
            (Ordering::Less, Ordering::Equal) => Direction::Left,
            (Ordering::Less, Ordering::Greater) => Direction::UpLeft,
            (Ordering::Equal, Ordering::Less) => Direction::Down,
            (Ordering::Equal, Ordering::Equal) => unimplemented!(),
            (Ordering::Equal, Ordering::Greater) => Direction::Up,
            (Ordering::Greater, Ordering::Less) => Direction::DownRight,
            (Ordering::Greater, Ordering::Equal) => Direction::Right,
            (Ordering::Greater, Ordering::Greater) => Direction::UpRight,
        }
    }
}

impl std::ops::Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Pos::new(self.x, self.y + 1),
            Direction::UpRight => Pos::new(self.x + 1, self.y + 1),
            Direction::Right => Pos::new(self.x + 1, self.y),
            Direction::DownRight => Pos::new(self.x + 1, self.y - 1),
            Direction::Down => Pos::new(self.x, self.y - 1),
            Direction::DownLeft => Pos::new(self.x - 1, self.y - 1),
            Direction::Left => Pos::new(self.x - 1, self.y),
            Direction::UpLeft => Pos::new(self.x - 1, self.y + 1),
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..8) {
            0 => Direction::Up,
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => unreachable!(),
        }
    }
}
