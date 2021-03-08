use crate::entity::EntityId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Tile {
    pub texture: TileTexture,
    pub agent: Option<EntityId>,
    // pub next_agent
}

impl Tile {
    pub fn new(texture: TileTexture) -> Tile {
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
pub enum TileTexture {
    Grass = 00,
    GrassTreeA = 01,
    GrassTreeB = 02,
    GrassRock = 03,
    Snow = 04,
    SnowTree = 05,
    SnowRock = 06,
    Water = 07,

    Sand = 08,
    SandPalm = 09,
    SandRock = 10,
    SandTreeDead = 11,
    Tundra = 12,
    TundraTreeDead = 13,
    TundraTree = 14,
    WaterRock = 15,

    Dirt = 16,
    DirtTreeDead = 17,
    DirtRock = 18,
    WoodHorizontal = 19,
    WoodVertical = 20,
    Mud = 21,
    MudDried = 22,
    Lava = 23,

    Sludge = 24,
    SludgeDried = 25,
    Rock = 26,
    Brick = 27,
}
