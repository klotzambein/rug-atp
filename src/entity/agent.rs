use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{
    tile::TileTexture,
    world::{Pos, World},
};

#[derive(Debug, Clone, Hash)]
pub struct Agent {
    pub job: Job,
    pub health: u8,
    pub cash: u32,
}

impl Agent {
    pub fn preferred_action(&self, pos: Pos, world: &World) -> AgentAction {
        match self.job {
            Job::None => AgentAction::None,
            Job::CompanyMember(_) => AgentAction::None,
            Job::Miner => AgentAction::None,
            Job::Farmer => AgentAction::None,
            Job::Explorer => {
                let dir: Direction = rand::random();

                let target = (pos + dir).wrap(world);

                let tt = world.tile_type(target);

                if tt == TileTexture::Grass {
                    AgentAction::Move(target)
                } else {
                    AgentAction::None
                }
            }
            Job::Fisher => AgentAction::None,
        }
    }
}

pub enum AgentAction {
    /// Do nothing this step
    None,
    /// Try to move to the given position
    Move(Pos),
    /// Mine a resource at the given position
    Farm(Pos),
    /// Enter a building at pos
    Enter(Pos),
}

// TODO: Move to company.
type CompanyId = u8;

#[derive(Debug, Clone, Hash)]
pub enum Job {
    None,
    CompanyMember(CompanyId),
    Miner,
    Farmer,
    Explorer,
    Fisher,
    // Butcher,
    // Builder,
}

impl Job {
    pub fn texture(&self) -> i32 {
        match self {
            Job::None => 0,
            Job::CompanyMember(c) => *c as i32 + 8,
            Job::Miner => 2,
            Job::Farmer => 3,
            Job::Explorer => 4,
            Job::Fisher => 5,
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
