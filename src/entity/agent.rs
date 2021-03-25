use std::cmp::Ordering;

use rand::{
    distributions::{Bernoulli, Standard},
    prelude::*,
    Rng,
};

use crate::world::{Pos, World};
use crate::market::{Market, Order};
use crate::entity::resources::*;

use super::{
    building::Building,
    resources::{PerResource, ResourceItem},
    Entity, EntityType,
};

#[derive(Debug, Clone, Hash)]
pub struct Agent {
    /// This contains the agents job, and all variables associated with said
    /// job.
    pub job: Job,
    /// This is the position of this agents hut
    pub home: Pos,
    /// Every agent has there own internal state machine, this contains the state.
    pub state: AgentState,
    /// This contains the nutritional value of each food resource. This value
    /// decreases when the food is eaten and increases when a different food is
    /// eaten.
    pub nutrition: PerResource<u8>,
    /// This contains the amount of resources the agent possesses at the moment.
    pub inventory: PerResource<u16>,
    /// This is the agent's goal for the day in terms of energy. It updates every day.
    /// The main goal is that the agent does not end the day with less energy than they 
    /// started. However, if their energy is below 5000 they are going to try and compensate
    /// for that
    pub energy_quota: u16,
    /// This is the agents current energy. This value is between 0 and 10_000
    pub energy: u16,
    /// This is the greed of the agent. It denotes the desired cash profit for each day
    /// It is initialized randomly from a normal distribution. It is initialized as an 
    /// integer to satisfy the Hash trait but in use it is divided by 100
    pub greed: u32,
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

        match self.state {
            AgentState::GoHome => match self.path_find(pos, Some(self.home), world) {
                Ok(h) => {
                    self.state = AgentState::BeHome;
                    self.update_energy_quota();
                    AgentAction::Enter(h)
                }
                Err(a) => a,
            },
            AgentState::BeHome => {
                if let Some(p) = world.find_tile_around(pos, 9, |p| world.tile_is_walkable(p)) {
                    // TODO: Make an informed choice
                    if random() {
                        self.state = AgentState::DoJob;
                    } else {
                        self.state = AgentState::GoToMarket(None);
                    }
                    AgentAction::Leave(p)
                } else {
                    AgentAction::None
                }
            }
            AgentState::DoJob => {
                if self.energy < 1000 {
                    self.state = AgentState::GoHome;
                }
                self.do_job(pos, world)
            }
            AgentState::GoToMarket(mut m) => {
                let action = self.find(world, pos, &mut m, |e| {
                    matches!(e.ty, EntityType::Building(Building::Market))
                });
                // the value of m can be changed by self.find, so we set the
                // state to the new value.
                self.state = AgentState::GoToMarket(m);
                match action {
                    Ok(h) => {
                        self.state = AgentState::TradeOnMarket;
                        AgentAction::Enter(h)
                    }
                    Err(a) => a,
                }
            }
            AgentState::TradeOnMarket => {
                if self.energy < 1000 {
                    if let Some(p) = world.find_tile_around(pos, 9, |p| world.tile_is_walkable(p)) {
                        // TODO: Maybe go straight to work
                        self.state = AgentState::GoHome;
                        AgentAction::Leave(p)
                    } else {
                        AgentAction::None
                    }
                } else {
                    self.trade_on_market(pos, world)
                }
            }
        }

        // if self.in_building {
        // if let Some(p) = world.find_tile_around(pos, 9, |p| world.tile_is_walkable(p)) {
        //     return AgentAction::Leave(p);
        // } else {
        //     return AgentAction::None;
        // }
        // }

        // self.do_job(pos, world)
    }

    pub fn do_job(&mut self, pos: Pos, world: &World) -> AgentAction {
        match &mut self.job {
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

    pub fn make_mealing_plan(&self, market: Market) -> Option<PerResource<u16>> {
        let mut to_ret: PerResource<u16> = PerResource::default();

        if self.energy >= self.energy_quota { return None; }
        
        // Calculating the energy needed to fulfill the quota and updating it as the meal plan is constructed
        let mut needed_energy = self.energy_quota - self.energy;
        

        // Finding the maximum projected energy over projected price (benefit) of each resource type on the market
        for r_item in ResourceItem::sorted(self, &market).iter() {

            // Calculating the energy gained by a single unit of that item
            // and the needed amount to fulfill the quota 
            let unit_energy = self.nutrition[*r_item] as u16;
            let needed_amount: u16 = needed_energy / unit_energy + 
                match needed_energy % unit_energy == 0 {
                    true => 0,
                    false => 1
                };

            // If the market or the inventory has more than the needed amount, 
            // we can buy it and the agent doesn't need anything else in its mealing plan
            let availability = market.availability(*r_item) + self.inventory[*r_item];
            if  availability >= needed_amount 
            {
                to_ret[*r_item] = needed_amount;
                return Some(to_ret);
            }

            // If the market does not have enough of the resource available, the agent buys whatever
            // is available and  the loop keeps going on other, less cost-efficient resources
            to_ret[*r_item] = availability;
            needed_energy -= needed_amount * unit_energy;
        }
        return Some(to_ret);
    }

    // Every time an agent gets home (finishes the working day), they set an energy quota
    // for the next day
    pub fn update_energy_quota(&mut self) -> () {
        let baseline = 5000;
        // If the agent's energy is above the baseline, their goal for the next day is simply not to
        // lose any more energy
        if self.energy >= 5000 {
            self.energy_quota = self.energy;
            return;
        }

        // Otherwise, the agent has to compensate - they need to increase their energy the next day 
        // by p%, where p is (5000 - energy) / 100
        let mut p: f32 = (baseline - self.energy) as f32;
        p /= 10000.0;

        let quota_f32 = (self.energy as f32) * (1.0 + p);
        self.energy_quota = quota_f32.ceil() as u16;
    }

    pub fn trade_on_market(&mut self, pos: Pos, world: &World) -> AgentAction {
        AgentAction::None
    }

    pub fn find(
        &mut self,
        world: &World,
        pos: Pos,
        target: &mut Option<Pos>,
        f: impl FnMut(&Entity) -> bool,
    ) -> Result<Pos, AgentAction> {
        let search_radius = 15;

        if target.is_none() {
            *target = world.find_entity_around(pos, search_radius * search_radius, f);
        }

        self.path_find(pos, *target, world)
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
            Err(a) => a,
        }
    }

    /// This function makes the agent path find towards a given position. Should
    /// no position be given the agent will walk around randomly.
    ///
    /// # Return value
    /// - Ok(pos) => The agent is right next to the target pos
    /// - Err(a) => a is either a move action, or if no move is possible a none
    ///   action
    pub fn path_find(
        &mut self,
        pos: Pos,
        target: Option<Pos>,
        world: &World,
    ) -> Result<Pos, AgentAction> {
        let mut rng = rand::thread_rng();
        let unstuckifier = Bernoulli::new(0.75).unwrap();

        if let Some(target) = target {
            if target.is_adjacent(pos) {
                return Ok(target);
            }
            if unstuckifier.sample(&mut rng) {
                let move_dir = Direction::delta(pos, target, world);
                let next_pos = (pos + move_dir).wrap(world);
                if world.tile_is_walkable(next_pos) {
                    return Err(AgentAction::Move(next_pos));
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
            Some(n) => Err(AgentAction::Move(n)),
            None => Err(AgentAction::None),
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

impl Default for Agent {
    fn default() -> Self {
        Agent {
            job: random(),
            state: AgentState::DoJob,
            home: Pos::default(),
            nutrition: PerResource::default(),
            inventory: PerResource::default(),
            energy: 5000,
            energy_quota: 5000,
            // TODO draw this from a normal distribution
            greed: 10,
            cash: 200,
            in_building: false,
            dead: false,
        }
    }
}

/// This keeps track of what the agent is currently doing.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AgentState {
    BeHome,
    GoHome,
    DoJob,
    GoToMarket(Option<Pos>),
    TradeOnMarket,
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
    // None,
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
    pub fn delta(end: Pos, start: Pos, world: &World) -> Direction {
        let mut dx = start.x.cmp(&end.x);
        let mut dy = start.y.cmp(&end.y);
        
        if (start.x - end.x).abs() > (world.width / 2) as i16 {
            dx = dx.reverse();    
        }
        if (start.y - end.y).abs() > (world.height / 2) as i16 {
            dy = dy.reverse();    
        }
        
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
