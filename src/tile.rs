pub struct Tile {
    pub texture: TileTexture,
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
