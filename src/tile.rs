use crate::entity::EntityId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Tile {
    pub texture: TileType,
    pub agent: Option<EntityId>,
    // pub next_agent
}

impl Tile {
    pub fn new(texture: TileType) -> Tile {
        Tile {
            texture,
            agent: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileAction {
    None,
    AgentClear,
    AgentSet(EntityId),
    /// Multiple actions on the same tile. Since we can not guarentee ordering we
    /// need to handle all conflicts seperatly, this contains an index into the
    /// conflict vector.
    Conflict(u16),
}

impl Default for TileAction {
    fn default() -> Self {
        TileAction::None
    }
}

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
    pub fn texture(self) -> i32 {
        match self {
            TileType::Grass => 00,
            TileType::GrassTreeA => 01,
            TileType::GrassTreeB => 02,
            TileType::GrassRock => 03,
            TileType::Snow => 04,
            TileType::SnowTree => 05,
            TileType::SnowRock => 06,
            TileType::Water => 07,
            TileType::Sand => 08,
            TileType::SandPalm => 09,
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

    pub fn walkable(self) -> bool {
        match self {
            TileType::Grass => true,
            TileType::GrassTreeA => false,
            TileType::GrassTreeB => false,
            TileType::GrassRock => false,
            TileType::Snow => true,
            TileType::SnowTree => false,
            TileType::SnowRock => false,
            TileType::Water => false,
            TileType::Sand => true,
            TileType::SandPalm => false,
            TileType::SandRock => false,
            TileType::SandTreeDead => false,
            TileType::Tundra => true,
            TileType::TundraTreeDead => false,
            TileType::TundraTree => false,
            TileType::WaterRock => false,
            TileType::Dirt => true,
            TileType::DirtTreeDead => false,
            TileType::DirtRock => false,
            TileType::WoodHorizontal => true,
            TileType::WoodVertical => true,
            TileType::Mud => true,
            TileType::MudDried => true,
            TileType::Lava => false,
            TileType::Sludge => false,
            TileType::SludgeDried => true,
            TileType::Rock => true,
            TileType::Brick => true,
        }
    }
}
