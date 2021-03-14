use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::world::{Pos, World};

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
        match self.job {
            Job::None => AgentAction::None,
            Job::Lumberer => {
                let _pos = world.find_tile_around(pos, 25, |p| match world.entity_at(p) {
                    Some(e) => match e.ty {
                        super::EntityType::Agent(_) => false,
                        super::EntityType::Resource(_) => true,
                        super::EntityType::Building(_) => false,
                    },
                    None => false,
                });
                AgentAction::None
            }
            Job::Farmer => AgentAction::None,
            Job::Explorer => {
                let dir: Direction = rand::random();

                let target = (pos + dir).wrap(world);

                if world.tile_is_walkable(target) {
                    AgentAction::Move(target)
                } else {
                    AgentAction::None
                }
            }
            Job::Fisher => AgentAction::None,
            Job::Butcher => AgentAction::None,
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
        Job::Explorer
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

impl std::ops::Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Pos(self.0, self.1 + 1),
            Direction::UpRight => Pos(self.0 + 1, self.1 + 1),
            Direction::Right => Pos(self.0 + 1, self.1),
            Direction::DownRight => Pos(self.0 + 1, self.1 - 1),
            Direction::Down => Pos(self.0, self.1 - 1),
            Direction::DownLeft => Pos(self.0 - 1, self.1 - 1),
            Direction::Left => Pos(self.0 - 1, self.1),
            Direction::UpLeft => Pos(self.0 - 1, self.1 + 1),
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
