use std::cmp::Ordering;

use rand::{
    distributions::{Bernoulli, Standard},
    prelude::*,
    Rng,
};

use crate::world::{Pos, World};

use super::{
    building::Building,
    resources::{Resource, ResourceItem},
    EntityType,
};

#[derive(Debug, Default, Clone, Hash)]
pub struct Agent {
    pub job: Job,
    pub nutrition_wheat: u8,
    pub nutrition_berry: u8,
    pub nutrition_meat: u8,
    pub nutrition_fish: u8,
    pub inventory_wheat: u8,
    pub inventory_berry: u8,
    pub inventory_meat: u8,
    pub inventory_fish: u8,
    pub energy: u8,
    pub cash: u32,
    pub target_pos: Option<Pos>,
    pub curr_dir: Option<Direction>,
}

impl Agent {
    pub fn step(&mut self, in_building: bool, pos: Pos, world: &World) -> AgentAction {
        if in_building {
            if let Some(p) = world.find_tile_around(pos, 9, |p| world.tile_is_walkable(p)) {
                return AgentAction::Leave(p);
            } else {
                return AgentAction::None;
            }
        }

        self.energy = self.energy.wrapping_sub(1);

        // if (self.energy <= 0) {

        // }
        match self.job {
            Job::None => AgentAction::None,
            Job::Lumberer => {
                let target_pos = world.find_entity_around(pos, 15 * 15, |e| {
                    matches!(e.ty, EntityType::Resource(Resource::Berry(_)))
                });

                let pf = self.path_find(pos, target_pos, world);

                match pf {
                    Ok(target) => AgentAction::Farm(target),
                    Err(Some(pos)) => AgentAction::Move(pos),
                    Err(None) => AgentAction::None,
                }
            }
            Job::Farmer => AgentAction::None,
            // Job::Farmer => FindAndFarm(Resource::Wheat(_))
            Job::Explorer => {
                // world.find_entity_around(pos, 15 * 15, |e| {
                //     matches!(e.ty, EntityType::Resource(Resource::Berry(_)))
                // });

                // world.find_entity_around(pos, 15 * 15, |e| {
                //     matches!(e.ty, EntityType::Resource(Resource::Wheat(_)))
                // });

                // world.find_entity_around(pos, 15 * 15, |e| {
                //     matches!(e.ty, EntityType::Resource(Resource::Meat(_)))
                // });

                // world.find_entity_around(pos, 15 * 15, |e| {
                //     matches!(e.ty, EntityType::Building(Building::Boat { agent: None }))
                // });

                // else

                let dir: Direction = rand::random();

                let target = (pos + dir).wrap(world);

                if world.tile_is_walkable(target) {
                    AgentAction::Move(target)
                } else {
                    AgentAction::None
                }
            }
            Job::Fisher => {
                // First find a boat
                let target_pos = world.find_entity_around(pos, 5 * 5, |e| {
                    matches!(e.ty, EntityType::Building(Building::Boat { agent: None }))
                });

                // if let Some

                AgentAction::None
            }
            Job::Butcher => AgentAction::None,
            // Job::Butcher => FindAndFarm(Resource::Meat(_))
        }
    }

    pub fn collect(&mut self, resource: ResourceItem) {
        match resource {
            ResourceItem::Wheat(n) => self.inventory_wheat += n,
            ResourceItem::Berry(n) => self.inventory_berry += n,
            ResourceItem::Fish(n) => self.inventory_fish += n,
            ResourceItem::Meat(n) => self.inventory_meat += n,
        }
    }

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
}

#[derive(Debug, Clone, Hash)]
pub enum Job {
    None,
    Explorer,
    Farmer,
    Lumberer,
    Fisher,
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
            Job::Explorer => 11,
            Job::Fisher => 12,
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
