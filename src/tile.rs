//! A tile is a 1x1 space in the world. Every tile has a type, and 1 or 0
//! entities on top.

use crate::entity::EntityId;

/// A tile is a 1x1 space in the world. Every tile has a type, and 1 or 0
/// entities on top.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Tile {
    pub texture: TileType,
    /// The id of the entity on top of the tile.
    pub agent: Option<EntityId>,
}

impl Tile {
    pub fn new(texture: TileType) -> Tile {
        Tile {
            texture,
            agent: None,
        }
    }
}

/// The tile types correspond to the texture in our sprite map, most of them are
/// unused.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum TileType {
    Grass,
    GrassTreeA,
    GrassTreeB,
    GrassRock,
    Snow,
    SnowTree,
    SnowRock,
    Water,

    Sand,
    SandPalm,
    SandRock,
    SandTreeDead,
    Tundra,
    TundraTreeDead,
    TundraTree,
    WaterRock,

    Dirt,
    DirtTreeDead,
    DirtRock,
    WoodHorizontal,
    WoodVertical,
    Mud,
    MudDried,
    Lava,

    Sludge,
    SludgeDried,
    Rock,
    Brick,
}

impl TileType {
    /// Return the texture index
    pub fn texture(self) -> i32 {
        match self {
            TileType::Grass => 0,
            TileType::GrassTreeA => 1,
            TileType::GrassTreeB => 2,
            TileType::GrassRock => 3,
            TileType::Snow => 4,
            TileType::SnowTree => 5,
            TileType::SnowRock => 6,
            TileType::Water => 7,
            TileType::Sand => 8,
            TileType::SandPalm => 9,
            TileType::SandRock => 10,
            TileType::SandTreeDead => 11,
            TileType::Tundra => 12,
            TileType::TundraTreeDead => 13,
            TileType::TundraTree => 14,
            TileType::WaterRock => 15,
            TileType::Dirt => 16,
            TileType::DirtTreeDead => 17,
            TileType::DirtRock => 18,
            TileType::WoodHorizontal => 19,
            TileType::WoodVertical => 20,
            TileType::Mud => 21,
            TileType::MudDried => 22,
            TileType::Lava => 23,
            TileType::Sludge => 24,
            TileType::SludgeDried => 25,
            TileType::Rock => 26,
            TileType::Brick => 27,
        }
    }

    /// True for tiles agents can walk on, if there is no entity on top.
    pub fn walkable(self) -> bool {
        match self {
            TileType::Grass
            | TileType::Snow
            | TileType::Sand
            | TileType::Tundra
            | TileType::Dirt
            | TileType::WoodHorizontal
            | TileType::WoodVertical
            | TileType::Mud
            | TileType::MudDried
            | TileType::SludgeDried
            | TileType::Rock
            | TileType::Brick => true,
            TileType::GrassTreeA
            | TileType::GrassTreeB
            | TileType::GrassRock
            | TileType::SnowTree
            | TileType::SnowRock
            | TileType::Water
            | TileType::SandPalm
            | TileType::SandRock
            | TileType::SandTreeDead
            | TileType::TundraTreeDead
            | TileType::TundraTree
            | TileType::WaterRock
            | TileType::DirtTreeDead
            | TileType::DirtRock
            | TileType::Lava
            | TileType::Sludge => false,
        }
    }
}
