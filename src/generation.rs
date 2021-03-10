//! Collection of utilities for world generation.

use noise::{NoiseFn, ScalePoint, SuperSimplex};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::{
    entity::{building::Building, EntityType},
    tile::TileTexture,
    world::Pos,
};

pub struct Biome {
    tiles: TileDistribution,
    /// To select what biome goes where we use two noise maps, one containing
    /// elevation, and one containing climate, from cold to warm.
    elevation: f64,
    climate: f64,
}

impl Biome {
    pub fn ocean() -> Biome {
        Biome {
            tiles: TileDistribution::ocean(),
            elevation: -1.,
            climate: 0.5,
        }
    }
    pub fn grass() -> Biome {
        Biome {
            tiles: TileDistribution::grass(),
            elevation: 0.75,
            climate: 0.7,
        }
    }
    pub fn desert() -> Biome {
        unimplemented!()
    }
    pub fn high_lands() -> Biome {
        unimplemented!()
    }
}

pub struct BiomeMap {
    biomes: Vec<Biome>,
    elevation: ScalePoint<SuperSimplex>,
    climate: ScalePoint<SuperSimplex>,
}

impl BiomeMap {
    pub fn new() -> BiomeMap {
        let elevation = ScalePoint::new(SuperSimplex::new()).set_all_scales(0.02, 0.02, 0.02, 0.02);
        let climate = ScalePoint::new(SuperSimplex::new()).set_all_scales(0.02, 0.02, 0.02, 0.02);
        BiomeMap {
            biomes: vec![
                Biome::ocean(),
                Biome::grass(),
                // Biome::desert(),
                // Biome::high_lands(),
            ],
            elevation,
            climate,
        }
    }

    pub fn get(&self, p: Pos, rng: &mut impl Rng) -> (TileTexture, Option<EntityType>) {
        let elevation = self.elevation.get([p.0 as f64, p.1 as f64]);
        let climate = self.climate.get([p.0 as f64, p.1 as f64]);
        let b = self
            .biomes
            .iter()
            .min_by_key(|b| {
                let de = b.elevation - elevation;
                let dc = b.climate - climate;
                ((de * de + dc * dc) * 1024.) as usize
            })
            .unwrap();

        b.tiles.sample(rng)
    }
}

pub struct TileDistribution {
    tiles: Vec<(TileTexture, Option<EntityType>)>,
    weights: WeightedIndex<u16>,
}

impl TileDistribution {
    pub fn grass() -> TileDistribution {
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
    pub fn ocean() -> TileDistribution {
        TileDistribution {
            tiles: vec![(TileTexture::Water, None), (TileTexture::WaterRock, None)],
            weights: WeightedIndex::new(&[1000, 1]).unwrap(),
        }
    }
}

impl Distribution<(TileTexture, Option<EntityType>)> for TileDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (TileTexture, Option<EntityType>) {
        self.tiles[self.weights.sample(rng)].clone()
    }
}
