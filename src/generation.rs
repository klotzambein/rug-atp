//! Collection of utilities for world generation.

use noise::{NoiseFn, ScalePoint, Seedable, SuperSimplex};
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::{
    entity::{building::Building, EntityType},
    tile::TileTexture,
    world::Pos,
};

const OCEAN_CUTOFF: isize = -300;
const BEACH_CUTOFF: isize = -150;

pub struct Biome {
    tiles: TileDistribution,
    /// To select what biome goes where we use two noise maps, one containing
    /// elevation, and one containing climate, from cold to warm. This function
    /// takes these two map scores and combines them into a final score for this
    /// biome. The highest scoring biome will be selected.
    score_fn: Box<dyn Fn(isize, isize) -> isize>,
}

impl Biome {
    pub fn ocean() -> Biome {
        Biome {
            tiles: TileDistribution::ocean(),
            score_fn: Box::new(|elevation, _climate| (elevation < OCEAN_CUTOFF) as isize * 10000),
        }
    }
    pub fn beach() -> Biome {
        Biome {
            tiles: TileDistribution::beach(),
            score_fn: Box::new(|elevation, _climate| (elevation < BEACH_CUTOFF) as isize * 9000),
        }
    }
    pub fn grass() -> Biome {
        Biome {
            tiles: TileDistribution::grass(),
            score_fn: Box::new(|_elevation, climate| 1000 - climate.abs()),
        }
    }
    pub fn desert() -> Biome {
        Biome {
            tiles: TileDistribution::desert(),
            score_fn: Box::new(|_elevation, climate| climate * 2 + 500),
        }
    }
    pub fn high_lands() -> Biome {
        Biome {
            tiles: TileDistribution::high_lands(),
            score_fn: Box::new(|elevation, climate| 500 - climate.abs() + elevation),
        }
    }
}

pub struct BiomeMap {
    biomes: Vec<Biome>,
    elevation: Vec<ScalePoint<SuperSimplex>>,
    climate: Vec<ScalePoint<SuperSimplex>>,
}

impl BiomeMap {
    pub fn new() -> BiomeMap {
        let elevation = [0.02, 0.04]
            .iter()
            .map(|s| {
                ScalePoint::new(SuperSimplex::new().set_seed(rand::random()))
                    .set_all_scales(*s, *s, *s, *s)
            })
            .collect();
        let climate = [0.01, 0.01]
            .iter()
            .map(|s| {
                ScalePoint::new(SuperSimplex::new().set_seed(rand::random()))
                    .set_all_scales(*s, *s, *s, *s)
            })
            .collect();
        BiomeMap {
            biomes: vec![
                Biome::ocean(),
                Biome::grass(),
                Biome::beach(),
                Biome::desert(),
                Biome::high_lands(),
            ],
            elevation,
            climate,
        }
    }

    pub fn get(&self, p: Pos, rng: &mut impl Rng) -> (TileTexture, Option<EntityType>) {
        let pos = [p.0 as f64 /  5., p.1 as f64 / 5.];
        let elevation = (self.elevation.iter().map(|e| e.get(pos)).sum::<f64>() * 500.) as isize;
        let climate = (self.climate.iter().map(|e| e.get(pos)).sum::<f64>() * 500.) as isize;
        let b = self
            .biomes
            .iter()
            .max_by_key(|b| (b.score_fn)(elevation, climate))
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
            weights: WeightedIndex::new(&[5000, 200, 30, 10]).unwrap(),
        }
    }

    pub fn high_lands() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileTexture::Dirt, None),
                (TileTexture::DirtRock, None),
                (TileTexture::DirtTreeDead, None),
                (TileTexture::Dirt, Some(EntityType::Building(Building::Hut))),
            ],
            weights: WeightedIndex::new(&[2000, 20, 10, 1]).unwrap(),
        }
    }

    pub fn ocean() -> TileDistribution {
        TileDistribution {
            tiles: vec![(TileTexture::Water, None), (TileTexture::WaterRock, None)],
            weights: WeightedIndex::new(&[1000, 1]).unwrap(),
        }
    }
    pub fn desert() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileTexture::Sand, None),
                (TileTexture::SandRock, None),
                (TileTexture::SandTreeDead, None),
                (
                    TileTexture::Sand,
                    Some(EntityType::Building(Building::Market)),
                ),
            ],
            weights: WeightedIndex::new(&[1000, 10, 15, 2]).unwrap(),
        }
    }
    pub fn beach() -> TileDistribution {
        TileDistribution {
            tiles: vec![
                (TileTexture::Sand, None),
                (TileTexture::SandPalm, None),
                (TileTexture::SandTreeDead, None),
            ],
            weights: WeightedIndex::new(&[1000, 20, 5]).unwrap(),
        }
    }
}

impl Distribution<(TileTexture, Option<EntityType>)> for TileDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> (TileTexture, Option<EntityType>) {
        self.tiles[self.weights.sample(rng)].clone()
    }
}
