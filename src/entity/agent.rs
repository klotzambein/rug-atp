use std::cmp::Ordering;

use rand::{
    distributions::{Bernoulli, Standard},
    prelude::*,
    Rng,
};

use crate::{
    config::Config,
    world::{Pos, World},
};
use crate::{market::Market, tile::TileType};

use super::{
    building::Building,
    resources::{PerResource, ResourceItem},
    Entity, EntityId, EntityType,
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
    pub inventory: PerResource<u32>,
    /// This contains the meal plan for the day
    pub shopping_list: Option<PerResource<u32>>,
    /// This contains the shopping list for the day
    pub meal_plan: Option<PerResource<u32>>,
    /// This is the agent's goal for the day in terms of energy. It updates every day.
    /// The main goal is that the agent does not end the day with less energy than they
    /// started. However, if their energy is below 5000 they are going to try and compensate
    /// for that
    pub energy_quota: u32,
    /// This is the agents current energy. This value is between 0 and 10_000
    pub energy: u32,
    /// This is the greed of the agent. It denotes the desired cash profit for each day
    /// It is initialized randomly from a normal distribution. It is initialized as an
    /// integer to satisfy the Hash trait but in use it is divided by 100
    pub greed: u32,
    /// This is the agents current cash. This can be used to buy resources at
    /// the market.
    pub cash: u32,
    // This is the cash that the agent needs to make
    pub cash_quota: u32,
    // Used to change jobs if quota is not met.
    pub timeout_quota: u16,
    /// This is true when the agent is in a building. To check which building
    /// the agent is in look up the current position in the world.
    pub in_building: bool,
    /// If this is true the agent is dead.
    pub dead: bool,
}

impl Agent {
    // TODO: change jobs after a while if they don't reach their targets
    pub fn step(&mut self, pos: Pos, world: &World) -> AgentAction {
        if self.dead {
            return AgentAction::None;
        }

        self.energy = self.energy.saturating_sub(world.config.energy_cost);
        if self.energy == 0 {
            return AgentAction::Die;
        }

        self.timeout_quota = self.timeout_quota.saturating_sub(1);

        match self.state {
            AgentState::GoHome => match self.path_find(pos, Some(self.home), world) {
                Ok(h) => {
                    self.state = AgentState::BeHome;
                    self.update_quotas(&world.config);
                    AgentAction::Enter(h)
                }
                Err(a) => {
                    if world.tile_type(pos) == TileType::Sand
                        && matches!(self.job, Job::Fisher { boat: Some(_) })
                    {
                        if let Some(p) =
                            world.find_tile_around(pos, 9, |p| world.tile_is_walkable(p))
                        {
                            AgentAction::LeaveBoat(p)
                        } else {
                            AgentAction::None
                        }
                    } else {
                        a
                    }
                }
            },
            AgentState::BeHome => {
                if self.energy < world.config.initial_energy {
                    if let Some(_meal_plan) = &self.meal_plan {
                        for r in ResourceItem::iterator() {
                            if _meal_plan[*r] > 0 && self.inventory[*r] > 0 {
                                let quantity: u32 = _meal_plan[*r].min(self.inventory[*r]);
                                return AgentAction::Consume(*r, quantity);
                            }
                        }
                        self.meal_plan = None;
                    }

                    // let mut items = self.nutrition.iter().collect::<Vec<_>>();
                    // items.sort_by_key(|i| std::cmp::Reverse(i.1));
                    // for (r, n) in items {
                    //     if *n > 0 && self.inventory[r] > 0 {
                    //         return AgentAction::Consume(r, self.inventory[r].min(1));
                    //     }
                    // }
                }
                if let Some(p) = world.find_tile_around(pos, 9, |p| self.can_walk_on(p, world)) {
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
                if self.energy < world.config.critical_energy
                    || world.time_of_day() > world.config.closing_time
                {
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
                if world.time_of_day() < world.config.closing_time {
                    if let Some(action) = self.trade_on_market(pos, world) {
                        return action;
                    }
                }
                // Leave the market
                if let Some(p) = world.find_tile_around(pos, 9, |p| self.can_walk_on(p, world)) {
                    // TODO: Maybe go straight to work
                    self.state = AgentState::GoHome;
                    AgentAction::Leave(p)
                } else {
                    AgentAction::None
                }
            }
        }
    }

    pub fn do_job(&mut self, pos: Pos, world: &World) -> AgentAction {
        match &mut self.job {
            Job::Lumberer => self.find_and_farm(world, pos, ResourceItem::Berry),
            Job::Farmer => self.find_and_farm(world, pos, ResourceItem::Wheat),
            Job::Butcher => self.find_and_farm(world, pos, ResourceItem::Meat),
            Job::Fisher { boat } => {
                if boat.is_some() {
                    // if on sand: go water
                    if let TileType::Sand = world.tile_type(pos) {
                        let target = world.find_tile_around(
                            pos,
                            world.config.search_radius * world.config.search_radius,
                            |p| world.tile_type(p) == TileType::Water,
                        );
                        return self
                            .path_find(pos, target, world)
                            .map(|p| {
                                if self.can_walk_on(p, world) {
                                    AgentAction::Move(p)
                                } else {
                                    AgentAction::None
                                }
                            })
                            .unwrap_or_else(|a| a);
                    } else {
                        // if on water
                        return self.find_and_farm(world, pos, ResourceItem::Fish);
                    }
                }

                // First find a boat and enter it
                let target_pos = world.find_entity_around(
                    pos,
                    world.config.search_radius * world.config.search_radius,
                    |e| matches!(e.ty, EntityType::Building(Building::Boat { .. })),
                );

                let pf = self.path_find(pos, target_pos, world);

                match pf {
                    Ok(p) => AgentAction::EnterBoat(p),
                    Err(a) => a,
                }
            }
            Job::Explorer {
                observations,
                count,
            } => {
                world.find_entity_around(
                    pos,
                    world.config.search_radius * world.config.search_radius,
                    |e| {
                        // matches!(e.ty, EntityType::Resource(Resource::Berry(_)))
                        match &e.ty {
                            EntityType::Resource(r) => {
                                observations[r.product()] +=
                                    r.available() as u32 / world.config.explorer_resource_divisor
                            }
                            EntityType::Building(Building::Boat { .. }) => {
                                observations.fish += world.config.explorer_fish_points
                            }
                            _ => (),
                        }
                        false
                    },
                );

                *count += 1;
                if *count == world.config.exploration_timeout {
                    let mut max_freq: u32 = 0;
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
                        ResourceItem::Fish => self.job = Job::Fisher { boat: None },
                        ResourceItem::Meat => self.job = Job::Butcher,
                    }
                }

                let dir: Direction = rand::random();

                let target = (pos + dir).wrap(world);

                if self.can_walk_on(target, world) {
                    AgentAction::Move(target)
                } else {
                    AgentAction::None
                }
            }
        }
    }

    pub fn make_mealing_plan(&self, market: &Market) -> Option<PerResource<u32>> {
        // return Some(PerResource::new(10));
        let mut to_ret: PerResource<u32> = PerResource::default();

        if self.energy >= self.energy_quota {
            return None;
        }

        // Calculating the energy needed to fulfill the quota and updating it as the meal plan is constructed
        let mut needed_energy = self.energy_quota.saturating_sub(self.energy);

        // Finding the maximum projected energy over projected price (benefit) of each resource type on the market
        for r_item in ResourceItem::sorted(self, &market).iter() {
            // Calculating the energy gained by a single unit of that item
            // and the needed amount to fulfill the quota
            let unit_energy = self.nutrition[*r_item] as u32;
            if unit_energy == 0 {
                continue;
            }
            let needed_amount: u32 =
                needed_energy / unit_energy + ((needed_energy % unit_energy != 0) as u32);

            // to_ret[*r_item] = needed_amount;

            // If the market or the inventory has more than the needed amount,
            // we can buy it and the agent doesn't need anything else in its mealing plan
            let availability = market.availability(*r_item) + self.inventory[*r_item];
            if availability >= needed_amount {
                to_ret[*r_item] = needed_amount;
                return Some(to_ret);
            }

            // If the market does not have enough of the resource available, the agent buys whatever
            // is available and  the loop keeps going on other, less cost-efficient resources
            to_ret[*r_item] = availability;
            needed_energy = needed_energy.saturating_sub(needed_amount * unit_energy);
        }
        return Some(to_ret);
    }

    // Every time an agent gets home (finishes the working day), they set an energy quota
    // for the next day
    pub fn update_quotas(&mut self, config: &Config) -> () {
        // If the agent's energy is above the baseline, their goal for the next day is simply not to
        // lose any more energy
        if self.energy >= config.initial_energy {
            self.energy_quota = self.energy;
            return;
        }

        // Otherwise, the agent has to compensate - they need to increase their energy the next day
        // by p%, where p is (5000 - energy) / 100
        let mut p: f32 = (config.initial_energy - self.energy) as f32;
        p /= 10000.0;

        let quota_f32 = (self.energy as f32) * (1.0 + p);
        self.energy_quota = quota_f32.ceil() as u32;

        if self.cash >= self.cash_quota {
            self.timeout_quota = config.timeout_quota;
        }

        if self.timeout_quota == 0 {
            self.job = Job::Explorer {
                count: 0,
                observations: Default::default(),
            };
            self.timeout_quota = config.timeout_quota;
        }

        // Update the cash quota with respect to the greed
        let desired_profit: f32 = (self.greed as f32) / 100.0;
        self.cash_quota = self.cash + ((self.cash as f32) * desired_profit) as u32;
    }

    fn make_shopping_list(&self, meal_plan: &Option<PerResource<u32>>) -> Option<PerResource<u32>> {
        // It subtracts the stuff they need from the stuff they have, so they don't buy excessively
        // If you need a product, you check how much of it you have and you put the rest on your shopping list
        let mut to_ret: PerResource<u32> = PerResource::default();

        if let Some(_meal_plan) = meal_plan {
            // Boolean flag about whether there is a single item on the shopping list
            let mut empty: bool = true;
            for r_item in ResourceItem::iterator() {
                // The item is only added to the shopping list if the agent currently has less than it needs
                if _meal_plan[*r_item] > self.inventory[*r_item] {
                    to_ret[*r_item] = _meal_plan[*r_item].saturating_sub(self.inventory[*r_item]);
                    empty = false;
                }
            }

            if empty {
                return None;
            }
            return Some(to_ret);
        } else {
            return None;
        }
    }

    //fn modify_shopping_item(&mut self, r_item: ResourceItem, amount: u32) {}

    pub fn trade_on_market(&mut self, _pos: Pos, world: &World) -> Option<AgentAction> {
        // If a shopping list is not constructed and the energy is below the quota, it constructs it
        // TODO update for multiple markets
        let market: &Market = &world.market;

        if let None = self.meal_plan {
            self.meal_plan = self.make_mealing_plan(market);
        }

        if let None = self.shopping_list {
            self.shopping_list = self.make_shopping_list(&self.meal_plan);
        }

        // After a shopping list has been constructed, it sells everything they don't need

        for r_item in ResourceItem::iterator() {
            let excess: u32 = match &self.meal_plan {
                Some(_meal_plan) => {
                    if self.inventory[*r_item] > _meal_plan[*r_item] {
                        self.inventory[*r_item] - _meal_plan[*r_item]
                    } else {
                        0
                    }
                }
                None => 0,
            };
            if excess == 0 {
                continue;
            }

            // It needs to calculate the total money spent on shopping
            let total_price: u32 = match &self.shopping_list {
                Some(_shopping_list) => market.total_price(_shopping_list),
                None => 0,
            };

            // After it has calculated the excess, it has to calculate the price needed to fulfill the quota
            let balance_after_purchase: u32 = if self.cash > total_price {
                self.cash - total_price
            } else {
                0
            };

            // insufficiency = 30   balance = 10 price = 3
            let insufficiency = self.cash_quota.saturating_sub(balance_after_purchase);
            if insufficiency > 0 {
                let price = insufficiency / excess + (insufficiency % excess != 0) as u32;
                let price = price.max(0);

                // Finally it puts the order on the action list
                return Some(AgentAction::MarketOrder {
                    item: *r_item,
                    price: price,
                    amount: excess,
                });
            }
        }

        // If the agent does not have enough money to fulfill their energy
        // quota, they will continuously try to buy stuff they can't afford
        // instead of selling something to get more money
        // TODO fix this
        // Finally it buys everything on the shopping list
        let mut action: AgentAction = AgentAction::None;
        let mut purchased_item: Option<ResourceItem> = None;

        if let Some(s_list) = &self.shopping_list {
            for r_item in ResourceItem::iterator() {
                if s_list[*r_item] == 0 {
                    continue;
                }
                purchased_item = Some(*r_item);
                action = AgentAction::MarketPurchase {
                    item: *r_item,
                    amount: s_list[*r_item],
                };
            }
        }

        // Remove the purchased item from the shopping list before returning
        // TODO Robin: once again, mutability issues that I can't figure out
        // I made shopping list and meal_plan fields in Agent but I realized them as Options
        // because it is more generalizable and elegant. But now I can't change the value of one of the
        // resources in the PerResource because I need to unpack it and the some mutability shit

        // If you want to try and fix this, what I want this code to do is set the amount of purchased_item
        // in self.shopping_plan to 0
        // let mut shopping_list_copy = self.shopping_list.clone();

        if let Some(list) = &mut self.shopping_list {
            if let Some(r_item) = purchased_item {
                list[r_item] = 0;
            }
            // If there was no item for purchase, the shopping list would be empty
            else {
                self.shopping_list = None;
            }
        }

        match action {
            AgentAction::None => None,
            _ => Some(action),
        }
    }

    pub fn find(
        &mut self,
        world: &World,
        pos: Pos,
        target: &mut Option<Pos>,
        f: impl FnMut(&Entity) -> bool,
    ) -> Result<Pos, AgentAction> {
        if target.is_none() {
            *target = world.find_entity_around(
                pos,
                world.config.search_radius * world.config.search_radius,
                f,
            );
        }

        self.path_find(pos, *target, world)
    }

    /// This function will return actions that lead to the agents locating a resource and farming it.
    pub fn find_and_farm(&mut self, world: &World, pos: Pos, item: ResourceItem) -> AgentAction {
        let target_pos = world.find_entity_around(
            pos,
            world.config.search_radius * world.config.search_radius,
            |e| {
                if let EntityType::Resource(r) = &e.ty {
                    r.produces_item(item) && r.available() > 0
                } else {
                    false
                }
            },
        );

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
        let unstuckifier = Bernoulli::new(world.config.unstuckifier_chance).unwrap();

        if let Some(target) = target {
            if target.is_adjacent(pos) {
                return Ok(target);
            }
            if unstuckifier.sample(&mut rng) {
                let move_dir = Direction::delta(pos, target, world);
                let next_pos = (pos + move_dir).wrap(world);
                if self.can_walk_on(next_pos, world) {
                    return Err(AgentAction::Move(next_pos));
                }
            }
        }

        let next = world
            .neighbors(pos)
            .iter()
            .cloned()
            .filter(|p| self.can_walk_on(*p, world))
            .choose(&mut rng);

        match next {
            Some(n) => Err(AgentAction::Move(n)),
            None => Err(AgentAction::None),
        }
    }

    pub fn can_walk_on(&mut self, pos: Pos, world: &World) -> bool {
        if let Job::Fisher { boat: Some(_) } = self.job {
            world.tile_is_sailable(pos)
        } else {
            world.tile_is_walkable(pos)
        }
    }

    /// This function will add the given resource to the agents inventory.
    pub fn collect(&mut self, resource: ResourceItem, amount: u32) {
        self.inventory[resource] += amount;
    }

    pub fn consume(&mut self, resource: ResourceItem, quantity: u32, config: &Config) {
        assert!(self.inventory[resource] > 0);
        self.inventory[resource] = self.inventory[resource].saturating_sub(quantity);
        self.energy += (self.nutrition[resource] as u32) * quantity;
        if self.energy > config.max_energy {
            self.energy = config.max_energy;
        }

        for (r, n) in self.nutrition.iter_mut() {
            if r == resource {
                *n = n.saturating_sub(config.nutrition_sub.saturating_mul(quantity.min(255) as u8));
            } else {
                *n = n.saturating_add(config.nutrition_add.saturating_mul(quantity.min(255) as u8));
            }
        }
    }

    pub fn new(config: &Config) -> Self {
        let greed = (thread_rng().sample::<f32, _>(rand_distr::StandardNormal) * config.greed_sd
            + config.greed_mean)
            .max(0.) as u32;
        Agent {
            job: random(),
            state: AgentState::DoJob,
            home: Pos::default(),
            nutrition: PerResource::new(config.initial_nutrition),
            inventory: PerResource::new(config.initial_inventory),
            energy: config.initial_energy,
            energy_quota: config.initial_energy,
            // TODO draw this from a normal distribution
            greed,
            meal_plan: None,
            shopping_list: None,
            cash: config.initial_cash,
            cash_quota: config.initial_cash,
            in_building: false,
            dead: false,
            timeout_quota: config.timeout_quota,
        }
    }
}

/// This keeps track of what the agent is currently doing.
#[derive(Debug, Clone, Hash)]
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
    EnterBoat(Pos),
    LeaveBoat(Pos),
    /// Consume a resource.
    Consume(ResourceItem, u32),
    /// This is only valid if an agent is in a market. This action will create
    /// an order at the given price with the specified amount
    MarketOrder {
        item: ResourceItem,
        price: u32,
        amount: u32,
    },
    /// This is only valid if an agent is in a market. This action will purchase
    /// the given item at the cheapest market price.
    MarketPurchase {
        item: ResourceItem,
        amount: u32,
    },
    /// Die: remove this agent from this agent from the world and set its dead
    /// flag to true.
    Die,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Job {
    // None,
    Explorer {
        count: u16,
        observations: PerResource<u32>,
    },
    Farmer,
    Lumberer,
    Fisher {
        boat: Option<EntityId>,
    },
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
            Job::Fisher { boat: None } => 12,
            Job::Fisher { boat: Some(_) } => 51,
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
        match rng.gen_range(0..=4) {
            0 => Job::Explorer {
                observations: Default::default(),
                count: 0,
            },
            1 => Job::Farmer,
            2 => Job::Butcher,
            3 => Job::Lumberer,
            4 => Job::Fisher { boat: None },
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
