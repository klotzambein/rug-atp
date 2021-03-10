//! Collection of utilities for world generation.

use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::{
    entity::{building::Building, EntityType},
    tile::TileTexture,
};

pub struct TileDistribution {
    tiles: Vec<(TileTexture, Option<EntityType>)>,
    weights: WeightedIndex<u16>,
}

impl TileDistribution {
    pub fn new_v1() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileTexture::Grass, None),
                (TileTexture::GrassRock, None),
                (
                    TileTexture::Grass,
                    Some(EntityType::Building(Building::Hut)),
                ),
                (
                    TileTexture::Grass,
                    Some(EntityType::Building(Building::Market)),
                ),
            ],
            weights: WeightedIndex::new(&[500, 200, 3, 1]).unwrap(),
        }
    }
}

impl Distribution<(TileTexture, Option<EntityType>)> for TileDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (TileTexture, Option<EntityType>) {
        self.tiles[self.weights.sample(rng)].clone()
    }
}
